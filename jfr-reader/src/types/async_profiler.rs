use crate::event::EventType;
use serde::Deserialize;
///Java Class
#[derive(Clone, Debug, Deserialize)]
pub struct Class {
    ///Class Loader
    #[serde(rename = "classLoader")]
    pub class_loader: Option<ClassLoader>,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<Symbol>,
    ///Package
    #[serde(rename = "package")]
    pub package: Option<Package>,
    ///Access Modifiers
    #[serde(rename = "modifiers")]
    pub modifiers: i32,
}
///Java Class Loader
#[derive(Clone, Debug, Deserialize)]
pub struct ClassLoader {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<Box<Class>>,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<Symbol>,
}
///Frame type
#[derive(Clone, Debug, Deserialize)]
pub struct FrameType {
    ///Description
    #[serde(rename = "description")]
    pub description: Option<String>,
}
///GC When
#[derive(Clone, Debug, Deserialize)]
pub struct GCWhen {
    ///When
    #[serde(rename = "when")]
    pub when: Option<String>,
}
///Log Level
#[derive(Clone, Debug, Deserialize)]
pub struct LogLevel {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///Java Method
#[derive(Clone, Debug, Deserialize)]
pub struct Method {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<Class>,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<Symbol>,
    ///Descriptor
    #[serde(rename = "descriptor")]
    pub descriptor: Option<Symbol>,
    ///Access Modifiers
    #[serde(rename = "modifiers")]
    pub modifiers: i32,
    ///Hidden
    #[serde(rename = "hidden")]
    pub hidden: bool,
}
///Package
#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<Symbol>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct StackFrame {
    ///Java Method
    #[serde(rename = "method")]
    pub method: Option<Method>,
    ///Line Number
    #[serde(rename = "lineNumber")]
    pub line_number: i32,
    ///Bytecode Index
    #[serde(rename = "bytecodeIndex")]
    pub bytecode_index: i32,
    ///Frame Type
    #[serde(rename = "type")]
    pub r#type: Option<FrameType>,
}
///Stacktrace
#[derive(Clone, Debug, Deserialize)]
pub struct StackTrace {
    ///Truncated
    #[serde(rename = "truncated")]
    pub truncated: bool,
    ///Stack Frames
    #[serde(rename = "frames")]
    pub frames: Vec<StackFrame>,
}
///Symbol
#[derive(Clone, Debug, Deserialize)]
pub struct Symbol {
    ///String
    #[serde(rename = "string")]
    pub string: Option<String>,
}
///Thread
#[derive(Clone, Debug, Deserialize)]
pub struct Thread {
    ///OS Thread Name
    #[serde(rename = "osName")]
    pub os_name: Option<String>,
    ///OS Thread Id
    #[serde(rename = "osThreadId")]
    pub os_thread_id: i64,
    ///Java Thread Name
    #[serde(rename = "javaName")]
    pub java_name: Option<String>,
    ///Java Thread Id
    #[serde(rename = "javaThreadId")]
    pub java_thread_id: i64,
}
///Java Thread State
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadState {
    ///Description
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///Virtual Space
#[derive(Clone, Debug, Deserialize)]
pub struct VirtualSpace {
    ///Start Address
    #[serde(rename = "start")]
    pub start: i64,
    ///Committed End Address
    #[serde(rename = "committedEnd")]
    pub committed_end: i64,
    ///Committed Size
    #[serde(rename = "committedSize")]
    pub committed_size: i64,
    ///Reserved End Address
    #[serde(rename = "reservedEnd")]
    pub reserved_end: i64,
    ///Reserved Size
    #[serde(rename = "reservedSize")]
    pub reserved_size: i64,
}
///All non-event types
#[derive(Clone, Debug, Deserialize)]
pub enum Types {
    Class(Class),
    ClassLoader(ClassLoader),
    FrameType(FrameType),
    GCWhen(GCWhen),
    LogLevel(LogLevel),
    Method(Method),
    Package(Package),
    StackFrame(StackFrame),
    StackTrace(StackTrace),
    Symbol(Symbol),
    Thread(Thread),
    ThreadState(ThreadState),
    VirtualSpace(VirtualSpace),
}
///ActiveRecording
#[derive(Clone, Debug, Deserialize)]
pub struct ActiveRecording {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Id
    #[serde(rename = "id")]
    pub id: i64,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Destination
    #[serde(rename = "destination")]
    pub destination: Option<String>,
    ///Max Age
    #[serde(rename = "maxAge")]
    pub max_age: i64,
    ///Max Size
    #[serde(rename = "maxSize")]
    pub max_size: i64,
    ///Start Time
    #[serde(rename = "recordingStart")]
    pub recording_start: i64,
    ///Recording Duration
    #[serde(rename = "recordingDuration")]
    pub recording_duration: i64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for ActiveRecording {
    const NAME: &'static str = "ActiveRecording";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///ActiveSetting
#[derive(Clone, Debug, Deserialize)]
pub struct ActiveSetting {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Event Id
    #[serde(rename = "id")]
    pub id: i64,
    ///Setting Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Setting Value
    #[serde(rename = "value")]
    pub value: Option<String>,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for ActiveSetting {
    const NAME: &'static str = "ActiveSetting";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///CPUInformation
#[derive(Clone, Debug, Deserialize)]
pub struct CPUInformation {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Type
    #[serde(rename = "cpu")]
    pub cpu: Option<String>,
    ///Description
    #[serde(rename = "description")]
    pub description: Option<String>,
    ///Sockets
    #[serde(rename = "sockets")]
    pub sockets: u32,
    ///Cores
    #[serde(rename = "cores")]
    pub cores: u32,
    ///Hardware Threads
    #[serde(rename = "hwThreads")]
    pub hw_threads: u32,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for CPUInformation {
    const NAME: &'static str = "CPUInformation";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///CPULoad
#[derive(Clone, Debug, Deserialize)]
pub struct CPULoad {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///JVM User
    #[serde(rename = "jvmUser")]
    pub jvm_user: f32,
    ///JVM System
    #[serde(rename = "jvmSystem")]
    pub jvm_system: f32,
    ///Machine Total
    #[serde(rename = "machineTotal")]
    pub machine_total: f32,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for CPULoad {
    const NAME: &'static str = "CPULoad";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///ExecutionSample
#[derive(Clone, Debug, Deserialize)]
pub struct ExecutionSample {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Thread
    #[serde(rename = "sampledThread")]
    pub sampled_thread: Option<Thread>,
    ///Stack Trace
    #[serde(rename = "stackTrace")]
    pub stack_trace: Option<StackTrace>,
    ///Thread State
    #[serde(rename = "state")]
    pub state: Option<ThreadState>,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for ExecutionSample {
    const NAME: &'static str = "ExecutionSample";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///GCHeapSummary
#[derive(Clone, Debug, Deserialize)]
pub struct GCHeapSummary {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///When
    #[serde(rename = "when")]
    pub when: Option<GCWhen>,
    ///Heap Space
    #[serde(rename = "heapSpace")]
    pub heap_space: Option<VirtualSpace>,
    ///Heap Used
    #[serde(rename = "heapUsed")]
    pub heap_used: u64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for GCHeapSummary {
    const NAME: &'static str = "GCHeapSummary";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///InitialSystemProperty
#[derive(Clone, Debug, Deserialize)]
pub struct InitialSystemProperty {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Key
    #[serde(rename = "key")]
    pub key: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: Option<String>,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for InitialSystemProperty {
    const NAME: &'static str = "InitialSystemProperty";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///JVMInformation
#[derive(Clone, Debug, Deserialize)]
pub struct JVMInformation {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///JVM Name
    #[serde(rename = "jvmName")]
    pub jvm_name: Option<String>,
    ///JVM Version
    #[serde(rename = "jvmVersion")]
    pub jvm_version: Option<String>,
    ///JVM Command Line Arguments
    #[serde(rename = "jvmArguments")]
    pub jvm_arguments: Option<String>,
    ///JVM Settings File Arguments
    #[serde(rename = "jvmFlags")]
    pub jvm_flags: Option<String>,
    ///Java Application Arguments
    #[serde(rename = "javaArguments")]
    pub java_arguments: Option<String>,
    ///JVM Start Time
    #[serde(rename = "jvmStartTime")]
    pub jvm_start_time: i64,
    ///Process Identifier
    #[serde(rename = "pid")]
    pub pid: i64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for JVMInformation {
    const NAME: &'static str = "JVMInformation";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///JavaMonitorEnter
#[derive(Clone, Debug, Deserialize)]
pub struct JavaMonitorEnter {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Monitor Class
    #[serde(rename = "monitorClass")]
    pub monitor_class: Option<Class>,
    ///Previous Monitor Owner
    #[serde(rename = "previousOwner")]
    pub previous_owner: Option<Thread>,
    ///Monitor Address
    #[serde(rename = "address")]
    pub address: i64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for JavaMonitorEnter {
    const NAME: &'static str = "JavaMonitorEnter";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///LiveObject
#[derive(Clone, Debug, Deserialize)]
pub struct LiveObject {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///objectClass
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Allocation Size
    #[serde(rename = "allocationSize")]
    pub allocation_size: i64,
    ///Allocation Time
    #[serde(rename = "allocationTime")]
    pub allocation_time: i64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for LiveObject {
    const NAME: &'static str = "LiveObject";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///Log
#[derive(Clone, Debug, Deserialize)]
pub struct Log {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Level
    #[serde(rename = "level")]
    pub level: Option<LogLevel>,
    ///Message
    #[serde(rename = "message")]
    pub message: Option<String>,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for Log {
    const NAME: &'static str = "Log";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///NativeLibrary
#[derive(Clone, Debug, Deserialize)]
pub struct NativeLibrary {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Base Address
    #[serde(rename = "baseAddress")]
    pub base_address: i64,
    ///Top Address
    #[serde(rename = "topAddress")]
    pub top_address: i64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for NativeLibrary {
    const NAME: &'static str = "NativeLibrary";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///OSInformation
#[derive(Clone, Debug, Deserialize)]
pub struct OSInformation {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///OS Version
    #[serde(rename = "osVersion")]
    pub os_version: Option<String>,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for OSInformation {
    const NAME: &'static str = "OSInformation";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///ObjectAllocationInNewTLAB
#[derive(Clone, Debug, Deserialize)]
pub struct ObjectAllocationInNewTLAB {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Object Class
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Allocation Size
    #[serde(rename = "allocationSize")]
    pub allocation_size: i64,
    ///TLAB Size
    #[serde(rename = "tlabSize")]
    pub tlab_size: i64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for ObjectAllocationInNewTLAB {
    const NAME: &'static str = "ObjectAllocationInNewTLAB";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///ObjectAllocationOutsideTLAB
#[derive(Clone, Debug, Deserialize)]
pub struct ObjectAllocationOutsideTLAB {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Object Class
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Allocation Size
    #[serde(rename = "allocationSize")]
    pub allocation_size: i64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for ObjectAllocationOutsideTLAB {
    const NAME: &'static str = "ObjectAllocationOutsideTLAB";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///ThreadPark
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadPark {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
    ///Class Parked On
    #[serde(rename = "parkedClass")]
    pub parked_class: Option<Class>,
    ///Park Timeout
    #[serde(rename = "timeout")]
    pub timeout: i64,
    ///Park Until
    #[serde(rename = "until")]
    pub until: i64,
    ///Address of Object Parked
    #[serde(rename = "address")]
    pub address: i64,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for ThreadPark {
    const NAME: &'static str = "ThreadPark";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///Window
#[derive(Clone, Debug, Deserialize)]
pub struct Window {
    #[serde(flatten)]
    common: crate::event::CommonFields<Thread, StackTrace>,
}
impl<'slf> EventType<'slf, Thread, StackTrace> for Window {
    const NAME: &'static str = "Window";
    fn common_fields(
        &'slf self,
    ) -> &'slf crate::event::CommonFields<Thread, StackTrace> {
        &self.common
    }
}
///All events
#[derive(Clone, Debug, Deserialize)]
pub enum Events {
    ActiveRecording(ActiveRecording),
    ActiveSetting(ActiveSetting),
    CPUInformation(CPUInformation),
    CPULoad(CPULoad),
    ExecutionSample(ExecutionSample),
    GCHeapSummary(GCHeapSummary),
    InitialSystemProperty(InitialSystemProperty),
    JVMInformation(JVMInformation),
    JavaMonitorEnter(JavaMonitorEnter),
    LiveObject(LiveObject),
    Log(Log),
    NativeLibrary(NativeLibrary),
    OSInformation(OSInformation),
    ObjectAllocationInNewTLAB(ObjectAllocationInNewTLAB),
    ObjectAllocationOutsideTLAB(ObjectAllocationOutsideTLAB),
    ThreadPark(ThreadPark),
    Window(Window),
}

