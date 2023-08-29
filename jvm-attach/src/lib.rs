// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JVM attachment.

use std::{
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
    time::Duration,
};
use thiserror::Error;

#[cfg(unix)]
use std::os::unix::net::UnixStream;

#[derive(Debug, Error)]
pub enum Error {
    #[error("parsing proc/pid/status file: {0}")]
    StatusParse(&'static str),

    #[error("reading integer: not a UTF-8 string")]
    IntegerReadBadString,

    #[error("reading integer: not an integer")]
    IntegerReadNotInteger,

    #[error("no socket to connect to")]
    NoSocket,

    #[error("command error: {0}")]
    CommandError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// This crate's result type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Establish a connection to a JVM via a socket.
///
/// This is the protocol used on Linux.
///
/// 1. Client creates a file named `.attach_pid<jvm_pid>` in a well-known
///    directory (probably `/tmp`).
/// 2. Client sends SIGQUIT to the JVM process.
/// 3. JVM sees that `.attach_pid<jvm_pid>` exists and creates a
///    `.java_pid<jvm_pid>` socket file.
/// 4. Client sends commands to socket.
///
/// A tricky aspect to implementing this correctly is PID and mounts
/// namespaces. The well-known directory (likely `/tmp`) could reside in a
/// different mount namespace from the caller. And the PID namespace could
/// be different. This means the client and JVM could disagree on both the
/// PID and the well-known filesystem path.
#[cfg(unix)]
#[derive(Debug)]
pub struct UnixSocketRequest {
    pid: i32,
    ns_pid: i32,
    attach_pid_path: Option<PathBuf>,
    socket_path: PathBuf,
}

#[cfg(unix)]
impl UnixSocketRequest {
    /// Request attachment to a PID.
    pub fn new(pid: i32) -> Result<Self, Error> {
        let procfs = "/proc";

        let ns_pid = namespace_pid(pid, procfs)?.get_or_default(pid);

        let pid_fs_root = PathBuf::from(procfs).join(pid.to_string()).join("root");
        let protocol_path = pid_fs_root.join("tmp");

        let attach_pid_path = protocol_path.join(format!(".attach_pid{}", ns_pid));
        let socket_path = protocol_path.join(format!(".java_pid{}", ns_pid));

        // TODO consider looking at file owner and removing if different.
        let attach_pid_path = if !attach_pid_path.exists() {
            // No special content. Just a placeholder file.
            std::fs::File::create(&attach_pid_path)?;

            Some(attach_pid_path)
        } else {
            None
        };

        Ok(Self {
            pid,
            ns_pid,
            attach_pid_path,
            socket_path,
        })
    }

    /// Attempt to connect to the JVM command and control socket.
    ///
    /// Consumes the instance. Returns a new type which can issue commands to the
    /// socket if the socket materializes.
    pub fn try_connect(mut self, max_wait_time: Duration) -> Result<UnixSocketConnection> {
        let now = std::time::Instant::now();
        let max_time = now + max_wait_time;
        const SIGNAL_INTERVAL: Duration = Duration::from_millis(100);
        let last_signal_time = now - SIGNAL_INTERVAL;

        loop {
            let now = std::time::Instant::now();

            if self.socket_path.exists() {
                // Check permissions first?

                // Make sure we can connect.
                UnixStream::connect(&self.socket_path)?;

                if let Some(p) = self.attach_pid_path.take() {
                    let _ = std::fs::remove_file(p);
                }

                return Ok(UnixSocketConnection {
                    socket_path: self.socket_path.clone(),
                });
            }

            if now >= max_time {
                return Err(Error::NoSocket);
            }

            // Send SIGQUIT if it has been long enough since the last one.
            if now - last_signal_time >= SIGNAL_INTERVAL {
                // TODO handle error?
                unsafe {
                    libc::kill(self.pid, libc::SIGQUIT);
                }
            }

            // If the socket doesn't exist, sleep and try later.
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

#[cfg(unix)]
impl Drop for UnixSocketRequest {
    fn drop(&mut self) {
        if let Some(path) = &self.attach_pid_path {
            let _ = std::fs::remove_file(path);
        }
    }
}

/// Response to a command invocation.
pub enum CommandResponse {
    /// Command execution failed. The error string is captured.
    Error(String),
    /// Command execution success. The raw socket is available to read the response.
    Success(UnixStream),
}

#[cfg(unix)]
#[derive(Debug)]
pub struct UnixSocketConnection {
    socket_path: PathBuf,
}

#[cfg(unix)]
impl UnixSocketConnection {
    /// Sends a command to the socket.
    ///
    /// Result reflects whether the socket I/O worked correctly.
    ///
    /// In the case of a successful command, the returned value holds a reference to the
    /// socket, which can be read from as appropriate.
    pub fn send_command(&self, command: &str, mut args: Vec<&str>) -> Result<CommandResponse> {
        // Command requests are <version> <command> <args>. Each is NULL terminated.
        // Over the wire we send up to 3 arguments. If provided fewer, we send empty
        // strings because that's the protocol.
        if args.len() < 3 {
            args.extend(std::iter::repeat("").take(3 - args.len()))
        }

        let request = std::iter::once("1")
            .chain([command].into_iter())
            .chain(args.into_iter().take(3))
            .flat_map(|s| {
                let mut bytes = s.as_bytes().to_vec();
                bytes.push(0);

                bytes.into_iter()
            })
            .collect::<Vec<_>>();

        let mut sock = UnixStream::connect(&self.socket_path)?;

        sock.write_all(&request)?;

        let status = Self::read_int(&mut sock)?;

        if status == 0 {
            Ok(CommandResponse::Success(sock))
        } else {
            let mut res = String::new();
            sock.read_to_string(&mut res)?;

            Ok(CommandResponse::Error(res))
        }
    }

    /// Send a command and read its output as a Vec<u8>.
    pub fn send_command_bytes(&self, command: &str, args: Vec<&str>) -> Result<Vec<u8>> {
        match self.send_command(command, args)? {
            CommandResponse::Success(mut sock) => {
                let mut res = vec![];
                sock.read_to_end(&mut res)?;

                Ok(res)
            }
            CommandResponse::Error(reason) => Err(Error::CommandError(reason)),
        }
    }

    /// Send a command and read its output as a string.
    pub fn send_command_string(&self, command: &str, args: Vec<&str>) -> Result<String> {
        match self.send_command(command, args)? {
            CommandResponse::Success(mut sock) => {
                let mut res = String::new();
                sock.read_to_string(&mut res)?;

                Ok(res)
            }
            CommandResponse::Error(reason) => Err(Error::CommandError(reason)),
        }
    }

    fn read_int(sock: &mut UnixStream) -> Result<i32> {
        let mut result = vec![];
        let mut buf = vec![0u8; 1];

        loop {
            let count = sock.read(&mut buf)?;
            if count == 0 {
                break;
            }

            if buf[0] == b'\n' {
                break;
            }

            result.push(buf[0]);
        }

        let s = std::str::from_utf8(&result).map_err(|_| Error::IntegerReadBadString)?;
        let v = i32::from_str(s).map_err(|_| Error::IntegerReadNotInteger)?;

        Ok(v)
    }
}

/// Represents the results of looking up a namespace pid.
#[derive(Clone, Copy, Debug)]
enum NamespacePid {
    /// No /proc/pid/status file.
    NoStatus,
    /// Found a pid.
    Namespace(i32),
    /// No pid found.
    NoPid,
}

impl NamespacePid {
    /// Get the resolved pid or return the provided default.
    pub fn get_or_default(&self, default: i32) -> i32 {
        match self {
            Self::NoStatus => default,
            Self::Namespace(pid) => *pid,
            Self::NoPid => default,
        }
    }
}

/// Attempt to resolve the PID of a process using that process's namespace PID view.
fn namespace_pid(pid: i32, procfs: &str) -> Result<NamespacePid> {
    let status_file = PathBuf::from(format!("{}/{}/status", procfs, pid));

    if !status_file.exists() {
        return Ok(NamespacePid::NoStatus);
    }

    let status = std::fs::read_to_string(&status_file)?;

    for line in status.lines() {
        if !line.starts_with("NSpid:") {
            continue;
        }

        let value = line
            .split(':')
            .nth(1)
            .ok_or(Error::StatusParse("invalid syntax for NSpid: entry"))?;
        let value = value.trim();

        let ns_pid =
            i32::from_str(value).map_err(|_| Error::StatusParse("NSpid value not an integer"))?;

        return Ok(NamespacePid::Namespace(ns_pid));
    }

    Ok(NamespacePid::NoPid)
}
