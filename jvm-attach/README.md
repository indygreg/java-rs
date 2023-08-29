# jvm-attach

This crate implements the virtual machine attachment protocol as used by Hotspot
JVMs. This is the protocol that `jcmd` uses under the hood.

Using this crate you can connect to a JVM and issue commands to it.
