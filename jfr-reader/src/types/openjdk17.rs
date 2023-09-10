// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::Deserialize;
///Bytecode Instruction
#[derive(Clone, Debug, Deserialize)]
pub struct Bytecode {
    ///Instruction
    #[serde(rename = "bytecode")]
    pub bytecode: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct CalleeMethod {
    ///Class
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    ///Method Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Method Descriptor
    #[serde(rename = "descriptor")]
    pub descriptor: Option<String>,
}
///Chunk Header
#[derive(Clone, Debug, Deserialize)]
pub struct ChunkHeader {
    ///Payload
    #[serde(rename = "payload")]
    pub payload: Vec<i8>,
}
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
    ///Hidden
    #[serde(rename = "hidden")]
    pub hidden: bool,
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
///Code Blob Type
#[derive(Clone, Debug, Deserialize)]
pub struct CodeBlobType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
///Compiler Phase Type
#[derive(Clone, Debug, Deserialize)]
pub struct CompilerPhaseType {
    ///Phase
    #[serde(rename = "phase")]
    pub phase: Option<String>,
}
///Compiler Type
#[derive(Clone, Debug, Deserialize)]
pub struct CompilerType {
    ///Compiler
    #[serde(rename = "compiler")]
    pub compiler: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct CopyFailed {
    ///Object Count
    #[serde(rename = "objectCount")]
    pub object_count: u64,
    ///First Failed Object Size
    #[serde(rename = "firstSize")]
    pub first_size: u64,
    ///Smallest Failed Object Size
    #[serde(rename = "smallestSize")]
    pub smallest_size: u64,
    ///Total Object Size
    #[serde(rename = "totalSize")]
    pub total_size: u64,
}
///Deoptimization Action
#[derive(Clone, Debug, Deserialize)]
pub struct DeoptimizationAction {
    ///Action
    #[serde(rename = "action")]
    pub action: Option<String>,
}
///Deoptimization Reason
#[derive(Clone, Debug, Deserialize)]
pub struct DeoptimizationReason {
    ///Reason
    #[serde(rename = "reason")]
    pub reason: Option<String>,
}
///Flag Value Origin
#[derive(Clone, Debug, Deserialize)]
pub struct FlagValueOrigin {
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<String>,
}
///Frame type
#[derive(Clone, Debug, Deserialize)]
pub struct FrameType {
    ///Description
    #[serde(rename = "description")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct G1EvacuationStatistics {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Total memory allocated by PLABs
    #[serde(rename = "allocated")]
    pub allocated: u64,
    ///Total memory wasted within PLABs due to alignment or refill
    #[serde(rename = "wasted")]
    pub wasted: u64,
    ///Total memory occupied by objects within PLABs
    #[serde(rename = "used")]
    pub used: u64,
    ///Total memory wasted due to allocation undo within PLABs
    #[serde(rename = "undoWaste")]
    pub undo_waste: u64,
    ///Total memory wasted at the end of regions due to refill
    #[serde(rename = "regionEndWaste")]
    pub region_end_waste: u64,
    ///Number of regions refilled
    #[serde(rename = "regionsRefilled")]
    pub regions_refilled: u32,
    ///Total memory allocated using direct allocation outside of PLABs
    #[serde(rename = "directAllocated")]
    pub direct_allocated: u64,
    ///Total memory occupied by objects in regions where evacuation failed
    #[serde(rename = "failureUsed")]
    pub failure_used: u64,
    ///Total memory left unused in regions where evacuation failed
    #[serde(rename = "failureWaste")]
    pub failure_waste: u64,
}
///G1 Heap Region Type
#[derive(Clone, Debug, Deserialize)]
pub struct G1HeapRegionType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
///G1 YC Type
#[derive(Clone, Debug, Deserialize)]
pub struct G1YCType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
///GC Cause
#[derive(Clone, Debug, Deserialize)]
pub struct GCCause {
    ///Cause
    #[serde(rename = "cause")]
    pub cause: Option<String>,
}
///GC Name
#[derive(Clone, Debug, Deserialize)]
pub struct GCName {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///GC Threshold Updater
#[derive(Clone, Debug, Deserialize)]
pub struct GCThresholdUpdater {
    ///Updater
    #[serde(rename = "updater")]
    pub updater: Option<String>,
}
///GC When
#[derive(Clone, Debug, Deserialize)]
pub struct GCWhen {
    ///When
    #[serde(rename = "when")]
    pub when: Option<String>,
}
///Inflation Cause
#[derive(Clone, Debug, Deserialize)]
pub struct InflateCause {
    ///Cause
    #[serde(rename = "cause")]
    pub cause: Option<String>,
}
///Metadata Type
#[derive(Clone, Debug, Deserialize)]
pub struct MetadataType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
///Metaspace Object Type
#[derive(Clone, Debug, Deserialize)]
pub struct MetaspaceObjectType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct MetaspaceSizes {
    ///Committed memory for this space
    #[serde(rename = "committed")]
    pub committed: u64,
    ///Bytes allocated by objects in the space
    #[serde(rename = "used")]
    pub used: u64,
    ///Reserved memory for this space
    #[serde(rename = "reserved")]
    pub reserved: u64,
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
///Module
#[derive(Clone, Debug, Deserialize)]
pub struct Module {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<Symbol>,
    ///Version
    #[serde(rename = "version")]
    pub version: Option<Symbol>,
    ///Location
    #[serde(rename = "location")]
    pub location: Option<Symbol>,
    ///Class Loader
    #[serde(rename = "classLoader")]
    pub class_loader: Option<ClassLoader>,
}
///Narrow Oop Mode
#[derive(Clone, Debug, Deserialize)]
pub struct NarrowOopMode {
    ///Mode
    #[serde(rename = "mode")]
    pub mode: Option<String>,
}
///Network Interface
#[derive(Clone, Debug, Deserialize)]
pub struct NetworkInterfaceName {
    ///Network Interface Name
    #[serde(rename = "networkInterface")]
    pub network_interface: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct ObjectSpace {
    ///Start address of the space
    #[serde(rename = "start")]
    pub start: u64,
    ///End address of the space
    #[serde(rename = "end")]
    pub end: u64,
    ///Bytes allocated by objects in the space
    #[serde(rename = "used")]
    pub used: u64,
    ///Size of the space
    #[serde(rename = "size")]
    pub size: u64,
}
///Old Object
#[derive(Clone, Debug, Deserialize)]
pub struct OldObject {
    ///Memory Address
    #[serde(rename = "address")]
    pub address: u64,
    ///Java Class
    #[serde(rename = "type")]
    pub r#type: Option<Class>,
    ///Object description
    #[serde(rename = "description")]
    pub description: Option<String>,
    ///Object referencing this object
    #[serde(rename = "referrer")]
    pub referrer: Option<Box<Reference>>,
}
///Old Object Array
#[derive(Clone, Debug, Deserialize)]
pub struct OldObjectArray {
    ///Size of array
    #[serde(rename = "size")]
    pub size: i32,
    ///Index in the array
    #[serde(rename = "index")]
    pub index: i32,
}
///Old Object Field
#[derive(Clone, Debug, Deserialize)]
pub struct OldObjectField {
    ///Name of field
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Field modifiers
    #[serde(rename = "modifiers")]
    pub modifiers: i16,
}
///GC Root
#[derive(Clone, Debug, Deserialize)]
pub struct OldObjectGcRoot {
    ///Root information
    #[serde(rename = "description")]
    pub description: Option<String>,
    ///The subsystem of origin for the root
    #[serde(rename = "system")]
    pub system: Option<OldObjectRootSystem>,
    ///The root type
    #[serde(rename = "type")]
    pub r#type: Option<OldObjectRootType>,
}
///GC Root System
#[derive(Clone, Debug, Deserialize)]
pub struct OldObjectRootSystem {
    ///System
    #[serde(rename = "system")]
    pub system: Option<String>,
}
///GC Root Type
#[derive(Clone, Debug, Deserialize)]
pub struct OldObjectRootType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
///Package
#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<Symbol>,
    ///Module
    #[serde(rename = "module")]
    pub module: Option<Module>,
    ///Exported
    #[serde(rename = "exported")]
    pub exported: bool,
}
///Reference
#[derive(Clone, Debug, Deserialize)]
pub struct Reference {
    ///Array or null if it is not an array
    #[serde(rename = "array")]
    pub array: Option<OldObjectArray>,
    ///Field or null if it is an array
    #[serde(rename = "field")]
    pub field: Option<OldObjectField>,
    ///Object holder for this reference
    #[serde(rename = "object")]
    pub object: Option<OldObject>,
    ///The object is this many hops away
    #[serde(rename = "skip")]
    pub skip: i32,
}
///Reference Type
#[derive(Clone, Debug, Deserialize)]
pub struct ReferenceType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
///Shenandoah Heap Region State
#[derive(Clone, Debug, Deserialize)]
pub struct ShenandoahHeapRegionState {
    ///State
    #[serde(rename = "state")]
    pub state: Option<String>,
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
    ///Java Thread Group
    #[serde(rename = "group")]
    pub group: Option<ThreadGroup>,
}
///Thread Group
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadGroup {
    ///Parent
    #[serde(rename = "parent")]
    pub parent: Option<Box<ThreadGroup>>,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///Java Thread State
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadState {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///VM Operation Type
#[derive(Clone, Debug, Deserialize)]
pub struct VMOperationType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct VirtualSpace {
    ///Start address of the virtual space
    #[serde(rename = "start")]
    pub start: u64,
    ///End address of the committed memory for the virtual space
    #[serde(rename = "committedEnd")]
    pub committed_end: u64,
    ///Size of the committed memory for the virtual space
    #[serde(rename = "committedSize")]
    pub committed_size: u64,
    ///End address of the reserved memory for the virtual space
    #[serde(rename = "reservedEnd")]
    pub reserved_end: u64,
    ///Size of the reserved memory for the virtual space
    #[serde(rename = "reservedSize")]
    pub reserved_size: u64,
}
///Z Page Type
#[derive(Clone, Debug, Deserialize)]
pub struct ZPageTypeType {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
///Z Statistics Counter
#[derive(Clone, Debug, Deserialize)]
pub struct ZStatisticsCounterType {
    ///Counter
    #[serde(rename = "counter")]
    pub counter: Option<String>,
}
///Z Statistics Sampler
#[derive(Clone, Debug, Deserialize)]
pub struct ZStatisticsSamplerType {
    ///Sampler
    #[serde(rename = "sampler")]
    pub sampler: Option<String>,
}
///AllocationRequiringGC
#[derive(Clone, Debug, Deserialize)]
pub struct AllocationRequiringGC {
    ///Pending GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Allocation Size
    #[serde(rename = "size")]
    pub size: u64,
}
///Revoked biases for all instances of a class
#[derive(Clone, Debug, Deserialize)]
pub struct BiasedLockClassRevocation {
    ///Class whose biased locks were revoked
    #[serde(rename = "revokedClass")]
    pub revoked_class: Option<Class>,
    ///Whether further biasing for instances of this class will be allowed
    #[serde(rename = "disableBiasing")]
    pub disable_biasing: bool,
    ///Safepoint Identifier
    #[serde(rename = "safepointId")]
    pub safepoint_id: u64,
}
///Revoked bias of object
#[derive(Clone, Debug, Deserialize)]
pub struct BiasedLockRevocation {
    ///Class of object whose biased lock was revoked
    #[serde(rename = "lockClass")]
    pub lock_class: Option<Class>,
    ///Safepoint Identifier
    #[serde(rename = "safepointId")]
    pub safepoint_id: u64,
    ///Thread owning the bias before revocation
    #[serde(rename = "previousOwner")]
    pub previous_owner: Option<Thread>,
}
///Revoked bias of object biased towards own thread
#[derive(Clone, Debug, Deserialize)]
pub struct BiasedLockSelfRevocation {
    ///Class of object whose biased lock was revoked
    #[serde(rename = "lockClass")]
    pub lock_class: Option<Class>,
}
///BooleanFlag
#[derive(Clone, Debug, Deserialize)]
pub struct BooleanFlag {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: bool,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///BooleanFlagChanged
#[derive(Clone, Debug, Deserialize)]
pub struct BooleanFlagChanged {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Old Value
    #[serde(rename = "oldValue")]
    pub old_value: bool,
    ///New Value
    #[serde(rename = "newValue")]
    pub new_value: bool,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///Characteristics and descriptions of the processor(s) the JVM is running on
#[derive(Clone, Debug, Deserialize)]
pub struct CPUInformation {
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
///Information about the recent CPU usage of the JVM process
#[derive(Clone, Debug, Deserialize)]
pub struct CPULoad {
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
///Information about the CPU time stamp mechanism / (RD)TSC
#[derive(Clone, Debug, Deserialize)]
pub struct CPUTimeStampCounter {
    ///Fast Time
    #[serde(rename = "fastTimeEnabled")]
    pub fast_time_enabled: bool,
    ///Trusted Platform
    #[serde(rename = "fastTimeAutoEnabled")]
    pub fast_time_auto_enabled: bool,
    ///OS Frequency
    #[serde(rename = "osFrequency")]
    pub os_frequency: i64,
    ///Fast Time Frequency
    #[serde(rename = "fastTimeFrequency")]
    pub fast_time_frequency: i64,
}
///ClassDefine
#[derive(Clone, Debug, Deserialize)]
pub struct ClassDefine {
    ///Defined Class
    #[serde(rename = "definedClass")]
    pub defined_class: Option<Class>,
    ///Defining Class Loader
    #[serde(rename = "definingClassLoader")]
    pub defining_class_loader: Option<ClassLoader>,
}
///ClassLoad
#[derive(Clone, Debug, Deserialize)]
pub struct ClassLoad {
    ///Loaded Class
    #[serde(rename = "loadedClass")]
    pub loaded_class: Option<Class>,
    ///Defining Class Loader
    #[serde(rename = "definingClassLoader")]
    pub defining_class_loader: Option<ClassLoader>,
    ///Initiating Class Loader
    #[serde(rename = "initiatingClassLoader")]
    pub initiating_class_loader: Option<ClassLoader>,
}
///ClassLoaderStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct ClassLoaderStatistics {
    ///Class Loader
    #[serde(rename = "classLoader")]
    pub class_loader: Option<ClassLoader>,
    ///Parent Class Loader
    #[serde(rename = "parentClassLoader")]
    pub parent_class_loader: Option<ClassLoader>,
    ///Pointer to the ClassLoaderData structure in the JVM
    #[serde(rename = "classLoaderData")]
    pub class_loader_data: u64,
    ///Number of loaded classes
    #[serde(rename = "classCount")]
    pub class_count: i64,
    ///Total size of all allocated metaspace chunks (each chunk has several blocks)
    #[serde(rename = "chunkSize")]
    pub chunk_size: u64,
    ///Total size of all allocated metaspace blocks (each chunk has several blocks)
    #[serde(rename = "blockSize")]
    pub block_size: u64,
    ///Number of hidden classes
    #[serde(rename = "hiddenClassCount")]
    pub hidden_class_count: i64,
    ///Total size of all allocated metaspace chunks for hidden classes (each chunk has several blocks)
    #[serde(rename = "hiddenChunkSize")]
    pub hidden_chunk_size: u64,
    ///Total size of all allocated metaspace blocks for hidden classes (each chunk has several blocks)
    #[serde(rename = "hiddenBlockSize")]
    pub hidden_block_size: u64,
}
///ClassLoadingStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct ClassLoadingStatistics {
    ///Number of classes loaded since JVM start
    #[serde(rename = "loadedClassCount")]
    pub loaded_class_count: i64,
    ///Number of classes unloaded since JVM start
    #[serde(rename = "unloadedClassCount")]
    pub unloaded_class_count: i64,
}
///ClassRedefinition
#[derive(Clone, Debug, Deserialize)]
pub struct ClassRedefinition {
    ///Redefined Class
    #[serde(rename = "redefinedClass")]
    pub redefined_class: Option<Class>,
    ///The number of times the class has changed
    #[serde(rename = "classModificationCount")]
    pub class_modification_count: i32,
    ///Class Redefinition Id
    #[serde(rename = "redefinitionId")]
    pub redefinition_id: u64,
}
///ClassUnload
#[derive(Clone, Debug, Deserialize)]
pub struct ClassUnload {
    ///Unloaded Class
    #[serde(rename = "unloadedClass")]
    pub unloaded_class: Option<Class>,
    ///Defining Class Loader
    #[serde(rename = "definingClassLoader")]
    pub defining_class_loader: Option<ClassLoader>,
}
///CodeCacheConfiguration
#[derive(Clone, Debug, Deserialize)]
pub struct CodeCacheConfiguration {
    ///Initial Size
    #[serde(rename = "initialSize")]
    pub initial_size: u64,
    ///Reserved Size
    #[serde(rename = "reservedSize")]
    pub reserved_size: u64,
    ///Non-nmethod Size
    #[serde(rename = "nonNMethodSize")]
    pub non_n_method_size: u64,
    ///Profiled Size
    #[serde(rename = "profiledSize")]
    pub profiled_size: u64,
    ///Non-profiled Size
    #[serde(rename = "nonProfiledSize")]
    pub non_profiled_size: u64,
    ///Expansion size
    #[serde(rename = "expansionSize")]
    pub expansion_size: u64,
    ///Minimum Block Length
    #[serde(rename = "minBlockLength")]
    pub min_block_length: u64,
    ///Start Address
    #[serde(rename = "startAddress")]
    pub start_address: u64,
    ///Reserved Top
    #[serde(rename = "reservedTopAddress")]
    pub reserved_top_address: u64,
}
///A code heap is full, this leads to disabling the compiler
#[derive(Clone, Debug, Deserialize)]
pub struct CodeCacheFull {
    ///Code Heap
    #[serde(rename = "codeBlobType")]
    pub code_blob_type: Option<CodeBlobType>,
    ///Start Address
    #[serde(rename = "startAddress")]
    pub start_address: u64,
    ///Commited Top
    #[serde(rename = "commitedTopAddress")]
    pub commited_top_address: u64,
    ///Reserved Top
    #[serde(rename = "reservedTopAddress")]
    pub reserved_top_address: u64,
    ///Entries
    #[serde(rename = "entryCount")]
    pub entry_count: i32,
    ///Methods
    #[serde(rename = "methodCount")]
    pub method_count: i32,
    ///Adaptors
    #[serde(rename = "adaptorCount")]
    pub adaptor_count: i32,
    ///Unallocated
    #[serde(rename = "unallocatedCapacity")]
    pub unallocated_capacity: u64,
    ///Full Count
    #[serde(rename = "fullCount")]
    pub full_count: i32,
    ///Code Cache Maximum Capacity
    #[serde(rename = "codeCacheMaxCapacity")]
    pub code_cache_max_capacity: u64,
}
///CodeCacheStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct CodeCacheStatistics {
    ///Code Heap
    #[serde(rename = "codeBlobType")]
    pub code_blob_type: Option<CodeBlobType>,
    ///Start Address
    #[serde(rename = "startAddress")]
    pub start_address: u64,
    ///Reserved Top
    #[serde(rename = "reservedTopAddress")]
    pub reserved_top_address: u64,
    ///Entries
    #[serde(rename = "entryCount")]
    pub entry_count: i32,
    ///Methods
    #[serde(rename = "methodCount")]
    pub method_count: i32,
    ///Adaptors
    #[serde(rename = "adaptorCount")]
    pub adaptor_count: i32,
    ///Unallocated
    #[serde(rename = "unallocatedCapacity")]
    pub unallocated_capacity: u64,
    ///Full Count
    #[serde(rename = "fullCount")]
    pub full_count: i32,
}
///CodeSweeperConfiguration
#[derive(Clone, Debug, Deserialize)]
pub struct CodeSweeperConfiguration {
    ///Code Sweeper Enabled
    #[serde(rename = "sweeperEnabled")]
    pub sweeper_enabled: bool,
    ///Code Cache Flushing Enabled
    #[serde(rename = "flushingEnabled")]
    pub flushing_enabled: bool,
    ///Sweep Threshold
    #[serde(rename = "sweepThreshold")]
    pub sweep_threshold: u64,
}
///CodeSweeperStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct CodeSweeperStatistics {
    ///Sweeps
    #[serde(rename = "sweepCount")]
    pub sweep_count: i32,
    ///Methods Reclaimed
    #[serde(rename = "methodReclaimedCount")]
    pub method_reclaimed_count: i32,
    ///Time Spent Sweeping
    #[serde(rename = "totalSweepTime")]
    pub total_sweep_time: u64,
    ///Peak Time Fraction Sweep
    #[serde(rename = "peakFractionTime")]
    pub peak_fraction_time: u64,
    ///Peak Time Full Sweep
    #[serde(rename = "peakSweepTime")]
    pub peak_sweep_time: u64,
}
///Results of method compilation attempts
#[derive(Clone, Debug, Deserialize)]
pub struct Compilation {
    ///Compilation Identifier
    #[serde(rename = "compileId")]
    pub compile_id: u32,
    ///Compiler
    #[serde(rename = "compiler")]
    pub compiler: Option<CompilerType>,
    ///Method
    #[serde(rename = "method")]
    pub method: Option<Method>,
    ///Compilation Level
    #[serde(rename = "compileLevel")]
    pub compile_level: u16,
    ///Succeeded
    #[serde(rename = "succeded")]
    pub succeded: bool,
    ///On Stack Replacement
    #[serde(rename = "isOsr")]
    pub is_osr: bool,
    ///Compiled Code Size
    #[serde(rename = "codeSize")]
    pub code_size: u64,
    ///Inlined Code Size
    #[serde(rename = "inlinedBytes")]
    pub inlined_bytes: u64,
}
///In case a JIT compilation failed, a compilation failure is triggered, reporting the reason
#[derive(Clone, Debug, Deserialize)]
pub struct CompilationFailure {
    ///Failure Message
    #[serde(rename = "failureMessage")]
    pub failure_message: Option<String>,
    ///Compilation Identifier
    #[serde(rename = "compileId")]
    pub compile_id: u32,
}
///CompilerConfiguration
#[derive(Clone, Debug, Deserialize)]
pub struct CompilerConfiguration {
    ///Thread Count
    #[serde(rename = "threadCount")]
    pub thread_count: i32,
    ///Tiered Compilation
    #[serde(rename = "tieredCompilation")]
    pub tiered_compilation: bool,
}
///Describes the result of a method inlining attempt
#[derive(Clone, Debug, Deserialize)]
pub struct CompilerInlining {
    ///Compilation Identifier
    #[serde(rename = "compileId")]
    pub compile_id: u32,
    ///Caller Method
    #[serde(rename = "caller")]
    pub caller: Option<Method>,
    ///Callee Method
    #[serde(rename = "callee")]
    pub callee: Option<CalleeMethod>,
    ///Succeeded
    #[serde(rename = "succeeded")]
    pub succeeded: bool,
    ///Message
    #[serde(rename = "message")]
    pub message: Option<String>,
    ///Bytecode Index
    #[serde(rename = "bci")]
    pub bci: i32,
}
///Describes various phases of the compilation process like inlining or string optimization related phases
#[derive(Clone, Debug, Deserialize)]
pub struct CompilerPhase {
    ///Compile Phase
    #[serde(rename = "phase")]
    pub phase: Option<CompilerPhaseType>,
    ///Compilation Identifier
    #[serde(rename = "compileId")]
    pub compile_id: u32,
    ///Phase Level
    #[serde(rename = "phaseLevel")]
    pub phase_level: u16,
}
///CompilerStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct CompilerStatistics {
    ///Compiled Methods
    #[serde(rename = "compileCount")]
    pub compile_count: i32,
    ///Bailouts
    #[serde(rename = "bailoutCount")]
    pub bailout_count: i32,
    ///Invalidated Compilations
    #[serde(rename = "invalidatedCount")]
    pub invalidated_count: i32,
    ///OSR Compilations
    #[serde(rename = "osrCompileCount")]
    pub osr_compile_count: i32,
    ///Standard Compilations
    #[serde(rename = "standardCompileCount")]
    pub standard_compile_count: i32,
    ///OSR Bytes Compiled
    #[serde(rename = "osrBytesCompiled")]
    pub osr_bytes_compiled: u64,
    ///Standard Bytes Compiled
    #[serde(rename = "standardBytesCompiled")]
    pub standard_bytes_compiled: u64,
    ///Compilation Resulting Size
    #[serde(rename = "nmethodsSize")]
    pub nmethods_size: u64,
    ///Compilation Resulting Code Size
    #[serde(rename = "nmethodCodeSize")]
    pub nmethod_code_size: u64,
    ///Peak Time
    #[serde(rename = "peakTimeSpent")]
    pub peak_time_spent: i64,
    ///Total time
    #[serde(rename = "totalTimeSpent")]
    pub total_time_spent: i64,
}
///Concurrent Mode failed
#[derive(Clone, Debug, Deserialize)]
pub struct ConcurrentModeFailure {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
}
///Data could not be copied out from a buffer, typically because of contention
#[derive(Clone, Debug, Deserialize)]
pub struct DataLoss {
    ///Amount lost data
    #[serde(rename = "amount")]
    pub amount: u64,
    ///Total lost amount for thread
    #[serde(rename = "total")]
    pub total: u64,
}
///Describes the detection of an uncommon situation in a compiled method which may lead to deoptimization of the method
#[derive(Clone, Debug, Deserialize)]
pub struct Deoptimization {
    ///Compilation Identifier
    #[serde(rename = "compileId")]
    pub compile_id: u32,
    ///Compiler
    #[serde(rename = "compiler")]
    pub compiler: Option<CompilerType>,
    ///Method
    #[serde(rename = "method")]
    pub method: Option<Method>,
    ///Line Number
    #[serde(rename = "lineNumber")]
    pub line_number: i32,
    ///Bytecode Index
    #[serde(rename = "bci")]
    pub bci: i32,
    ///Instruction
    #[serde(rename = "instruction")]
    pub instruction: Option<Bytecode>,
    ///Reason
    #[serde(rename = "reason")]
    pub reason: Option<DeoptimizationReason>,
    ///Action
    #[serde(rename = "action")]
    pub action: Option<DeoptimizationAction>,
}
///DoubleFlag
#[derive(Clone, Debug, Deserialize)]
pub struct DoubleFlag {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: f64,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///DoubleFlagChanged
#[derive(Clone, Debug, Deserialize)]
pub struct DoubleFlagChanged {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Old Value
    #[serde(rename = "oldValue")]
    pub old_value: f64,
    ///New Value
    #[serde(rename = "newValue")]
    pub new_value: f64,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///Who requested the recording and why
#[derive(Clone, Debug, Deserialize)]
pub struct DumpReason {
    ///Reason for writing recording data to disk
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    ///Id of the recording that triggered the dump, or -1 if it was not related to a recording
    #[serde(rename = "recordingId")]
    pub recording_id: i32,
}
///Evacuation of an object failed
#[derive(Clone, Debug, Deserialize)]
pub struct EvacuationFailed {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Evacuation Failed Data
    #[serde(rename = "evacuationFailed")]
    pub evacuation_failed: Option<CopyFailed>,
}
///EvacuationInformation
#[derive(Clone, Debug, Deserialize)]
pub struct EvacuationInformation {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Collection Set Regions
    #[serde(rename = "cSetRegions")]
    pub c_set_regions: u32,
    ///Memory usage before GC in the collection set regions
    #[serde(rename = "cSetUsedBefore")]
    pub c_set_used_before: u64,
    ///Memory usage after GC in the collection set regions
    #[serde(rename = "cSetUsedAfter")]
    pub c_set_used_after: u64,
    ///Regions chosen as allocation regions during evacuation (includes survivors and old space regions)
    #[serde(rename = "allocationRegions")]
    pub allocation_regions: u32,
    ///Memory usage before GC in allocation regions
    #[serde(rename = "allocationRegionsUsedBefore")]
    pub allocation_regions_used_before: u64,
    ///Memory usage after GC in allocation regions
    #[serde(rename = "allocationRegionsUsedAfter")]
    pub allocation_regions_used_after: u64,
    ///Bytes Copied
    #[serde(rename = "bytesCopied")]
    pub bytes_copied: u64,
    ///Regions Freed
    #[serde(rename = "regionsFreed")]
    pub regions_freed: u32,
}
///Execution of a VM Operation
#[derive(Clone, Debug, Deserialize)]
pub struct ExecuteVMOperation {
    ///Operation
    #[serde(rename = "operation")]
    pub operation: Option<VMOperationType>,
    ///If the operation occured at a safepoint
    #[serde(rename = "safepoint")]
    pub safepoint: bool,
    ///If the calling thread was blocked until the operation was complete
    #[serde(rename = "blocking")]
    pub blocking: bool,
    ///Thread requesting operation. If non-blocking, will be set to 0 indicating thread is unknown
    #[serde(rename = "caller")]
    pub caller: Option<Thread>,
    ///The safepoint (if any) under which this operation was completed
    #[serde(rename = "safepointId")]
    pub safepoint_id: u64,
}
///Snapshot of a threads state
#[derive(Clone, Debug, Deserialize)]
pub struct ExecutionSample {
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
///Flush
#[derive(Clone, Debug, Deserialize)]
pub struct Flush {
    ///Flush Identifier
    #[serde(rename = "flushId")]
    pub flush_id: u64,
    ///Elements Written
    #[serde(rename = "elements")]
    pub elements: u64,
    ///Size Written
    #[serde(rename = "size")]
    pub size: u64,
}
///Statistics related to current adaptive IHOP calculation
#[derive(Clone, Debug, Deserialize)]
pub struct G1AdaptiveIHOP {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Current IHOP Threshold
    #[serde(rename = "threshold")]
    pub threshold: u64,
    ///Current IHOP threshold in percent of the internal target occupancy
    #[serde(rename = "thresholdPercentage")]
    pub threshold_percentage: f32,
    ///Internal target old generation occupancy to reach at the start of mixed GC
    #[serde(rename = "ihopTargetOccupancy")]
    pub ihop_target_occupancy: u64,
    ///Current old generation occupancy
    #[serde(rename = "currentOccupancy")]
    pub current_occupancy: u64,
    ///Additional buffer size
    #[serde(rename = "additionalBufferSize")]
    pub additional_buffer_size: u64,
    ///Current predicted allocation rate for the mutator in bytes/second
    #[serde(rename = "predictedAllocationRate")]
    pub predicted_allocation_rate: f64,
    ///Current predicted time from the end of the last concurrent start to the first mixed GC
    #[serde(rename = "predictedMarkingDuration")]
    pub predicted_marking_duration: i64,
    ///Indicates whether the adaptive IHOP prediction is active
    #[serde(rename = "predictionActive")]
    pub prediction_active: bool,
}
///Basic statistics related to current IHOP calculation
#[derive(Clone, Debug, Deserialize)]
pub struct G1BasicIHOP {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Current IHOP threshold
    #[serde(rename = "threshold")]
    pub threshold: u64,
    ///Current IHOP threshold in percent of old generation
    #[serde(rename = "thresholdPercentage")]
    pub threshold_percentage: f32,
    ///Target old generation occupancy to reach at the start of mixed GC
    #[serde(rename = "targetOccupancy")]
    pub target_occupancy: u64,
    ///Current old generation occupancy
    #[serde(rename = "currentOccupancy")]
    pub current_occupancy: u64,
    ///Mutator allocation during mutator operation in the most recent interval
    #[serde(rename = "recentMutatorAllocationSize")]
    pub recent_mutator_allocation_size: u64,
    ///Time the mutator ran in the most recent interval
    #[serde(rename = "recentMutatorDuration")]
    pub recent_mutator_duration: i64,
    ///Allocation rate of the mutator in the most recent interval in bytes/second
    #[serde(rename = "recentAllocationRate")]
    pub recent_allocation_rate: f64,
    ///Last time from the end of the last concurrent start to the first mixed GC
    #[serde(rename = "lastMarkingDuration")]
    pub last_marking_duration: i64,
}
///Memory related evacuation statistics during GC for the old generation
#[derive(Clone, Debug, Deserialize)]
pub struct G1EvacuationOldStatistics {
    ///Evacuation Statistics
    #[serde(rename = "statistics")]
    pub statistics: Option<G1EvacuationStatistics>,
}
///Memory related evacuation statistics during GC for the young generation
#[derive(Clone, Debug, Deserialize)]
pub struct G1EvacuationYoungStatistics {
    ///Evacuation Statistics
    #[serde(rename = "statistics")]
    pub statistics: Option<G1EvacuationStatistics>,
}
///Extra information specific to G1 Young Garbage Collections
#[derive(Clone, Debug, Deserialize)]
pub struct G1GarbageCollection {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<G1YCType>,
}
///Information about a specific heap region in the G1 GC
#[derive(Clone, Debug, Deserialize)]
pub struct G1HeapRegionInformation {
    ///Index
    #[serde(rename = "index")]
    pub index: u32,
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<G1HeapRegionType>,
    ///Start
    #[serde(rename = "start")]
    pub start: u64,
    ///Used
    #[serde(rename = "used")]
    pub used: u64,
}
///Information about a G1 heap region type change
#[derive(Clone, Debug, Deserialize)]
pub struct G1HeapRegionTypeChange {
    ///Index
    #[serde(rename = "index")]
    pub index: u32,
    ///From
    #[serde(rename = "from")]
    pub from: Option<G1HeapRegionType>,
    ///To
    #[serde(rename = "to")]
    pub to: Option<G1HeapRegionType>,
    ///Start
    #[serde(rename = "start")]
    pub start: u64,
    ///Used
    #[serde(rename = "used")]
    pub used: u64,
}
///G1HeapSummary
#[derive(Clone, Debug, Deserialize)]
pub struct G1HeapSummary {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///When
    #[serde(rename = "when")]
    pub when: Option<GCWhen>,
    ///Eden Used Size
    #[serde(rename = "edenUsedSize")]
    pub eden_used_size: u64,
    ///Eden Total Size
    #[serde(rename = "edenTotalSize")]
    pub eden_total_size: u64,
    ///Survivor Used Size
    #[serde(rename = "survivorUsedSize")]
    pub survivor_used_size: u64,
    ///Number of Regions
    #[serde(rename = "numberOfRegions")]
    pub number_of_regions: u32,
}
///G1MMU
#[derive(Clone, Debug, Deserialize)]
pub struct G1MMU {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Time slice used to calculate MMU
    #[serde(rename = "timeSlice")]
    pub time_slice: i64,
    ///Time stopped because of GC during last time slice
    #[serde(rename = "gcTime")]
    pub gc_time: i64,
    ///Max time allowed to be spent on GC during last time slice
    #[serde(rename = "pauseTarget")]
    pub pause_target: i64,
}
///The configuration of the garbage collector
#[derive(Clone, Debug, Deserialize)]
pub struct GCConfiguration {
    ///The garbage collector used for the young generation
    #[serde(rename = "youngCollector")]
    pub young_collector: Option<GCName>,
    ///The garbage collector used for the old generation
    #[serde(rename = "oldCollector")]
    pub old_collector: Option<GCName>,
    ///Number of parallel threads to use for garbage collection
    #[serde(rename = "parallelGCThreads")]
    pub parallel_gc_threads: u32,
    ///Number of concurrent threads to use for garbage collection
    #[serde(rename = "concurrentGCThreads")]
    pub concurrent_gc_threads: u32,
    ///Whether a dynamic number of GC threads are used or not
    #[serde(rename = "usesDynamicGCThreads")]
    pub uses_dynamic_gc_threads: bool,
    ///Whether System.gc() is concurrent or not
    #[serde(rename = "isExplicitGCConcurrent")]
    pub is_explicit_gc_concurrent: bool,
    ///Whether System.gc() will cause a garbage collection or not
    #[serde(rename = "isExplicitGCDisabled")]
    pub is_explicit_gc_disabled: bool,
    ///Target for GC pauses
    #[serde(rename = "pauseTarget")]
    pub pause_target: i64,
    ///Target for runtime vs garbage collection time
    #[serde(rename = "gcTimeRatio")]
    pub gc_time_ratio: u32,
}
///The configuration of the garbage collected heap
#[derive(Clone, Debug, Deserialize)]
pub struct GCHeapConfiguration {
    ///Minimum Heap Size
    #[serde(rename = "minSize")]
    pub min_size: u64,
    ///Maximum Heap Size
    #[serde(rename = "maxSize")]
    pub max_size: u64,
    ///Initial Heap Size
    #[serde(rename = "initialSize")]
    pub initial_size: u64,
    ///If compressed Oops (Ordinary Object Pointers) are enabled
    #[serde(rename = "usesCompressedOops")]
    pub uses_compressed_oops: bool,
    ///The kind of compressed oops being used
    #[serde(rename = "compressedOopsMode")]
    pub compressed_oops_mode: Option<NarrowOopMode>,
    ///Object alignment (in bytes) on the heap
    #[serde(rename = "objectAlignment")]
    pub object_alignment: u64,
    ///Heap Address Size (in bits)
    #[serde(rename = "heapAddressBits")]
    pub heap_address_bits: u8,
}
///GCHeapSummary
#[derive(Clone, Debug, Deserialize)]
pub struct GCHeapSummary {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///When
    #[serde(rename = "when")]
    pub when: Option<GCWhen>,
    ///Heap Space
    #[serde(rename = "heapSpace")]
    pub heap_space: Option<VirtualSpace>,
    ///Bytes allocated by objects in the heap
    #[serde(rename = "heapUsed")]
    pub heap_used: u64,
}
///GCLocker
#[derive(Clone, Debug, Deserialize)]
pub struct GCLocker {
    ///The number of Java threads in a critical section when the GC locker is started
    #[serde(rename = "lockCount")]
    pub lock_count: u32,
    ///The number of Java threads stalled by the GC locker
    #[serde(rename = "stallCount")]
    pub stall_count: u32,
}
///GCPhaseConcurrent
#[derive(Clone, Debug, Deserialize)]
pub struct GCPhaseConcurrent {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///GCPhaseConcurrentLevel1
#[derive(Clone, Debug, Deserialize)]
pub struct GCPhaseConcurrentLevel1 {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///GC phases for parallel workers
#[derive(Clone, Debug, Deserialize)]
pub struct GCPhaseParallel {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///GC Worker Identifier
    #[serde(rename = "gcWorkerId")]
    pub gc_worker_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///GCPhasePause
#[derive(Clone, Debug, Deserialize)]
pub struct GCPhasePause {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///GCPhasePauseLevel1
#[derive(Clone, Debug, Deserialize)]
pub struct GCPhasePauseLevel1 {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///GCPhasePauseLevel2
#[derive(Clone, Debug, Deserialize)]
pub struct GCPhasePauseLevel2 {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///GCPhasePauseLevel3
#[derive(Clone, Debug, Deserialize)]
pub struct GCPhasePauseLevel3 {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///GCPhasePauseLevel4
#[derive(Clone, Debug, Deserialize)]
pub struct GCPhasePauseLevel4 {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///Total count of processed references during GC
#[derive(Clone, Debug, Deserialize)]
pub struct GCReferenceStatistics {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<ReferenceType>,
    ///Total Count
    #[serde(rename = "count")]
    pub count: u64,
}
///The configuration of the survivors of garbage collection
#[derive(Clone, Debug, Deserialize)]
pub struct GCSurvivorConfiguration {
    ///Upper limit for the age of how old objects to keep in the survivor area
    #[serde(rename = "maxTenuringThreshold")]
    pub max_tenuring_threshold: u8,
    ///Initial age limit for how old objects to keep in survivor area
    #[serde(rename = "initialTenuringThreshold")]
    pub initial_tenuring_threshold: u8,
}
///The configuration of the Thread Local Allocation Buffers (TLABs)
#[derive(Clone, Debug, Deserialize)]
pub struct GCTLABConfiguration {
    ///If Thread Local Allocation Buffers (TLABs) are in use
    #[serde(rename = "usesTLABs")]
    pub uses_tla_bs: bool,
    ///Minimum TLAB Size
    #[serde(rename = "minTLABSize")]
    pub min_tlab_size: u64,
    ///TLAB Refill Waste Limit
    #[serde(rename = "tlabRefillWasteLimit")]
    pub tlab_refill_waste_limit: u64,
}
///Garbage collection performed by the JVM
#[derive(Clone, Debug, Deserialize)]
pub struct GarbageCollection {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///The name of the Garbage Collector
    #[serde(rename = "name")]
    pub name: Option<GCName>,
    ///The reason for triggering this Garbage Collection
    #[serde(rename = "cause")]
    pub cause: Option<GCCause>,
    ///Sum of all the times in which Java execution was paused during the garbage collection
    #[serde(rename = "sumOfPauses")]
    pub sum_of_pauses: u64,
    ///Longest individual pause during the garbage collection
    #[serde(rename = "longestPause")]
    pub longest_pause: u64,
}
///HeapDump
#[derive(Clone, Debug, Deserialize)]
pub struct HeapDump {
    ///Destination
    #[serde(rename = "destination")]
    pub destination: Option<String>,
    ///Size
    #[serde(rename = "size")]
    pub size: i64,
    ///GC Before Dump
    #[serde(rename = "gcBeforeDump")]
    pub gc_before_dump: bool,
    ///On Out of Memory Error
    #[serde(rename = "onOutOfMemoryError")]
    pub on_out_of_memory_error: bool,
}
///Key-value pairs for environment variables at JVM startup
#[derive(Clone, Debug, Deserialize)]
pub struct InitialEnvironmentVariable {
    ///Key
    #[serde(rename = "key")]
    pub key: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: Option<String>,
}
///System Property at JVM start
#[derive(Clone, Debug, Deserialize)]
pub struct InitialSystemProperty {
    ///Key
    #[serde(rename = "key")]
    pub key: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: Option<String>,
}
///IntFlag
#[derive(Clone, Debug, Deserialize)]
pub struct IntFlag {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: i32,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///IntFlagChanged
#[derive(Clone, Debug, Deserialize)]
pub struct IntFlagChanged {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Old Value
    #[serde(rename = "oldValue")]
    pub old_value: i32,
    ///New Value
    #[serde(rename = "newValue")]
    pub new_value: i32,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///Restart of the JIT compilers after they were stopped
#[derive(Clone, Debug, Deserialize)]
pub struct JITRestart {
    ///Freed Memory
    #[serde(rename = "freedMemory")]
    pub freed_memory: i32,
    ///Code Cache Maximum Capacity
    #[serde(rename = "codeCacheMaxCapacity")]
    pub code_cache_max_capacity: u64,
}
///Description of JVM and the Java application
#[derive(Clone, Debug, Deserialize)]
pub struct JVMInformation {
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
///JavaMonitorEnter
#[derive(Clone, Debug, Deserialize)]
pub struct JavaMonitorEnter {
    ///Monitor Class
    #[serde(rename = "monitorClass")]
    pub monitor_class: Option<Class>,
    ///Previous Monitor Owner
    #[serde(rename = "previousOwner")]
    pub previous_owner: Option<Thread>,
    ///Monitor Address
    #[serde(rename = "address")]
    pub address: u64,
}
///JavaMonitorInflate
#[derive(Clone, Debug, Deserialize)]
pub struct JavaMonitorInflate {
    ///Monitor Class
    #[serde(rename = "monitorClass")]
    pub monitor_class: Option<Class>,
    ///Monitor Address
    #[serde(rename = "address")]
    pub address: u64,
    ///Cause of inflation
    #[serde(rename = "cause")]
    pub cause: Option<InflateCause>,
}
///Waiting on a Java monitor
#[derive(Clone, Debug, Deserialize)]
pub struct JavaMonitorWait {
    ///Class of object waited on
    #[serde(rename = "monitorClass")]
    pub monitor_class: Option<Class>,
    ///Notifying Thread
    #[serde(rename = "notifier")]
    pub notifier: Option<Thread>,
    ///Maximum wait time
    #[serde(rename = "timeout")]
    pub timeout: i64,
    ///Wait has been timed out
    #[serde(rename = "timedOut")]
    pub timed_out: bool,
    ///Address of object waited on
    #[serde(rename = "address")]
    pub address: u64,
}
///JavaThreadStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct JavaThreadStatistics {
    ///Number of live active threads including both daemon and non-daemon threads
    #[serde(rename = "activeCount")]
    pub active_count: i64,
    ///Number of live daemon threads
    #[serde(rename = "daemonCount")]
    pub daemon_count: i64,
    ///Number of threads created and also started since JVM start
    #[serde(rename = "accumulatedCount")]
    pub accumulated_count: i64,
    ///Peak live thread count since JVM start or when peak count was reset
    #[serde(rename = "peakCount")]
    pub peak_count: i64,
}
///LoaderConstraintsTableStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct LoaderConstraintsTableStatistics {
    ///Bucket Count
    #[serde(rename = "bucketCount")]
    pub bucket_count: u64,
    ///Number of all entries
    #[serde(rename = "entryCount")]
    pub entry_count: u64,
    ///Total memory footprint (the table itself plus all of the entries)
    #[serde(rename = "totalFootprint")]
    pub total_footprint: u64,
    ///The maximum bucket length (entries in a single bucket)
    #[serde(rename = "bucketCountMaximum")]
    pub bucket_count_maximum: u64,
    ///The average bucket length
    #[serde(rename = "bucketCountAverage")]
    pub bucket_count_average: f32,
    ///How far bucket lengths are spread out from their average value
    #[serde(rename = "bucketCountVariance")]
    pub bucket_count_variance: f32,
    ///How far bucket lengths are spread out from their mean (expected) value
    #[serde(rename = "bucketCountStandardDeviation")]
    pub bucket_count_standard_deviation: f32,
    ///How many items were added since last event (per second)
    #[serde(rename = "insertionRate")]
    pub insertion_rate: f32,
    ///How many items were removed since last event (per second)
    #[serde(rename = "removalRate")]
    pub removal_rate: f32,
}
///LongFlag
#[derive(Clone, Debug, Deserialize)]
pub struct LongFlag {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: i64,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///LongFlagChanged
#[derive(Clone, Debug, Deserialize)]
pub struct LongFlagChanged {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Old Value
    #[serde(rename = "oldValue")]
    pub old_value: i64,
    ///New Value
    #[serde(rename = "newValue")]
    pub new_value: i64,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///MetaspaceAllocationFailure
#[derive(Clone, Debug, Deserialize)]
pub struct MetaspaceAllocationFailure {
    ///Class Loader
    #[serde(rename = "classLoader")]
    pub class_loader: Option<ClassLoader>,
    ///Hidden Class Loader
    #[serde(rename = "hiddenClassLoader")]
    pub hidden_class_loader: bool,
    ///Size
    #[serde(rename = "size")]
    pub size: u64,
    ///Metadata Type
    #[serde(rename = "metadataType")]
    pub metadata_type: Option<MetadataType>,
    ///Metaspace Object Type
    #[serde(rename = "metaspaceObjectType")]
    pub metaspace_object_type: Option<MetaspaceObjectType>,
}
///MetaspaceChunkFreeListSummary
#[derive(Clone, Debug, Deserialize)]
pub struct MetaspaceChunkFreeListSummary {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///When
    #[serde(rename = "when")]
    pub when: Option<GCWhen>,
    ///Metadata Type
    #[serde(rename = "metadataType")]
    pub metadata_type: Option<MetadataType>,
    ///Specialized Chunks
    #[serde(rename = "specializedChunks")]
    pub specialized_chunks: u64,
    ///Specialized Chunks Total Size
    #[serde(rename = "specializedChunksTotalSize")]
    pub specialized_chunks_total_size: u64,
    ///Small Chunks
    #[serde(rename = "smallChunks")]
    pub small_chunks: u64,
    ///Small Chunks Total Size
    #[serde(rename = "smallChunksTotalSize")]
    pub small_chunks_total_size: u64,
    ///Medium Chunks
    #[serde(rename = "mediumChunks")]
    pub medium_chunks: u64,
    ///Medium Chunks Total Size
    #[serde(rename = "mediumChunksTotalSize")]
    pub medium_chunks_total_size: u64,
    ///Humongous Chunks
    #[serde(rename = "humongousChunks")]
    pub humongous_chunks: u64,
    ///Humongous Chunks Total Size
    #[serde(rename = "humongousChunksTotalSize")]
    pub humongous_chunks_total_size: u64,
}
///MetaspaceGCThreshold
#[derive(Clone, Debug, Deserialize)]
pub struct MetaspaceGCThreshold {
    ///Old Value
    #[serde(rename = "oldValue")]
    pub old_value: u64,
    ///New Value
    #[serde(rename = "newValue")]
    pub new_value: u64,
    ///Updater
    #[serde(rename = "updater")]
    pub updater: Option<GCThresholdUpdater>,
}
///MetaspaceOOM
#[derive(Clone, Debug, Deserialize)]
pub struct MetaspaceOOM {
    ///Class Loader
    #[serde(rename = "classLoader")]
    pub class_loader: Option<ClassLoader>,
    ///Hidden Class Loader
    #[serde(rename = "hiddenClassLoader")]
    pub hidden_class_loader: bool,
    ///Size
    #[serde(rename = "size")]
    pub size: u64,
    ///Metadata Type
    #[serde(rename = "metadataType")]
    pub metadata_type: Option<MetadataType>,
    ///Metaspace Object Type
    #[serde(rename = "metaspaceObjectType")]
    pub metaspace_object_type: Option<MetaspaceObjectType>,
}
///MetaspaceSummary
#[derive(Clone, Debug, Deserialize)]
pub struct MetaspaceSummary {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///When
    #[serde(rename = "when")]
    pub when: Option<GCWhen>,
    ///GC Threshold
    #[serde(rename = "gcThreshold")]
    pub gc_threshold: u64,
    ///Total
    #[serde(rename = "metaspace")]
    pub metaspace: Option<MetaspaceSizes>,
    ///Data
    #[serde(rename = "dataSpace")]
    pub data_space: Option<MetaspaceSizes>,
    ///Class
    #[serde(rename = "classSpace")]
    pub class_space: Option<MetaspaceSizes>,
}
///ModuleExport
#[derive(Clone, Debug, Deserialize)]
pub struct ModuleExport {
    ///Exported Package
    #[serde(rename = "exportedPackage")]
    pub exported_package: Option<Package>,
    ///Module to which the package is qualifiedly exported. If null or N/A, the package is unqualifiedly exported
    #[serde(rename = "targetModule")]
    pub target_module: Option<Module>,
}
///A directed edge representing a dependency
#[derive(Clone, Debug, Deserialize)]
pub struct ModuleRequire {
    ///Source Module
    #[serde(rename = "source")]
    pub source: Option<Module>,
    ///Required Module
    #[serde(rename = "requiredModule")]
    pub required_module: Option<Module>,
}
///NativeLibrary
#[derive(Clone, Debug, Deserialize)]
pub struct NativeLibrary {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Starting address of the module
    #[serde(rename = "baseAddress")]
    pub base_address: u64,
    ///Ending address of the module
    #[serde(rename = "topAddress")]
    pub top_address: u64,
}
///Snapshot of a threads state when in native
#[derive(Clone, Debug, Deserialize)]
pub struct NativeMethodSample {
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
///NetworkUtilization
#[derive(Clone, Debug, Deserialize)]
pub struct NetworkUtilization {
    ///Network Interface Name
    #[serde(rename = "networkInterface")]
    pub network_interface: Option<NetworkInterfaceName>,
    ///Number of incoming bits per second
    #[serde(rename = "readRate")]
    pub read_rate: i64,
    ///Number of outgoing bits per second
    #[serde(rename = "writeRate")]
    pub write_rate: i64,
}
///Description of the OS the JVM runs on, for example, a uname-like output
#[derive(Clone, Debug, Deserialize)]
pub struct OSInformation {
    ///OS Version
    #[serde(rename = "osVersion")]
    pub os_version: Option<String>,
}
///Allocation in new Thread Local Allocation Buffer
#[derive(Clone, Debug, Deserialize)]
pub struct ObjectAllocationInNewTLAB {
    ///Class of allocated object
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Allocation Size
    #[serde(rename = "allocationSize")]
    pub allocation_size: u64,
    ///TLAB Size
    #[serde(rename = "tlabSize")]
    pub tlab_size: u64,
}
///Allocation outside Thread Local Allocation Buffers
#[derive(Clone, Debug, Deserialize)]
pub struct ObjectAllocationOutsideTLAB {
    ///Class of allocated object
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Allocation Size
    #[serde(rename = "allocationSize")]
    pub allocation_size: u64,
}
///ObjectAllocationSample
#[derive(Clone, Debug, Deserialize)]
pub struct ObjectAllocationSample {
    ///Class of allocated object
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///The relative weight of the sample. Aggregating the weights for a large number of samples, for a particular class, thread or stack trace, gives a statistically accurate representation of the allocation pressure
    #[serde(rename = "weight")]
    pub weight: i64,
}
///ObjectCount
#[derive(Clone, Debug, Deserialize)]
pub struct ObjectCount {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Object Class
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Count
    #[serde(rename = "count")]
    pub count: i64,
    ///Total Size
    #[serde(rename = "totalSize")]
    pub total_size: u64,
}
///ObjectCountAfterGC
#[derive(Clone, Debug, Deserialize)]
pub struct ObjectCountAfterGC {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Object Class
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Count
    #[serde(rename = "count")]
    pub count: i64,
    ///Total Size
    #[serde(rename = "totalSize")]
    pub total_size: u64,
}
///Extra information specific to Old Garbage Collections
#[derive(Clone, Debug, Deserialize)]
pub struct OldGarbageCollection {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
}
///A potential memory leak
#[derive(Clone, Debug, Deserialize)]
pub struct OldObjectSample {
    ///Allocation Time
    #[serde(rename = "allocationTime")]
    pub allocation_time: u64,
    ///Object Age
    #[serde(rename = "objectAge")]
    pub object_age: u64,
    ///Last Known Heap Usage
    #[serde(rename = "lastKnownHeapUsage")]
    pub last_known_heap_usage: u64,
    ///Object
    #[serde(rename = "object")]
    pub object: Option<OldObject>,
    ///If the object is an array, the number of elements, or -1 if it is not an array
    #[serde(rename = "arrayElements")]
    pub array_elements: i32,
    ///GC Root
    #[serde(rename = "root")]
    pub root: Option<OldObjectGcRoot>,
}
///PSHeapSummary
#[derive(Clone, Debug, Deserialize)]
pub struct PSHeapSummary {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///When
    #[serde(rename = "when")]
    pub when: Option<GCWhen>,
    ///Old Space
    #[serde(rename = "oldSpace")]
    pub old_space: Option<VirtualSpace>,
    ///Old Object Space
    #[serde(rename = "oldObjectSpace")]
    pub old_object_space: Option<ObjectSpace>,
    ///Young Space
    #[serde(rename = "youngSpace")]
    pub young_space: Option<VirtualSpace>,
    ///Eden Space
    #[serde(rename = "edenSpace")]
    pub eden_space: Option<ObjectSpace>,
    ///From Space
    #[serde(rename = "fromSpace")]
    pub from_space: Option<ObjectSpace>,
    ///To Space
    #[serde(rename = "toSpace")]
    pub to_space: Option<ObjectSpace>,
}
///Extra information specific to Parallel Old Garbage Collections
#[derive(Clone, Debug, Deserialize)]
pub struct ParallelOldGarbageCollection {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///The address of the dense prefix, used when compacting
    #[serde(rename = "densePrefix")]
    pub dense_prefix: u64,
}
///OS Physical Memory
#[derive(Clone, Debug, Deserialize)]
pub struct PhysicalMemory {
    ///Total amount of physical memory available to OS
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    ///Total amount of physical memory in use
    #[serde(rename = "usedSize")]
    pub used_size: u64,
}
///PlaceholderTableStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct PlaceholderTableStatistics {
    ///Number of buckets
    #[serde(rename = "bucketCount")]
    pub bucket_count: u64,
    ///Number of all entries
    #[serde(rename = "entryCount")]
    pub entry_count: u64,
    ///Total memory footprint (the table itself plus all of the entries)
    #[serde(rename = "totalFootprint")]
    pub total_footprint: u64,
    ///The maximum bucket length (entries in a single bucket)
    #[serde(rename = "bucketCountMaximum")]
    pub bucket_count_maximum: u64,
    ///The average bucket length
    #[serde(rename = "bucketCountAverage")]
    pub bucket_count_average: f32,
    ///How far bucket lengths are spread out from their average value
    #[serde(rename = "bucketCountVariance")]
    pub bucket_count_variance: f32,
    ///How far bucket lengths are spread out from their mean (expected) value
    #[serde(rename = "bucketCountStandardDeviation")]
    pub bucket_count_standard_deviation: f32,
    ///How many items were added since last event (per second)
    #[serde(rename = "insertionRate")]
    pub insertion_rate: f32,
    ///How many items were removed since last event (per second)
    #[serde(rename = "removalRate")]
    pub removal_rate: f32,
}
///Object survived scavenge and was copied to a new Promotion Local Allocation Buffer (PLAB). Supported GCs are Parallel Scavange, G1 and CMS with Parallel New. Due to promotion being done in parallel an object might be reported multiple times as the GC threads race to copy all objects.
#[derive(Clone, Debug, Deserialize)]
pub struct PromoteObjectInNewPLAB {
    ///Identifier signifying GC during which the object was promoted
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Class of promoted object
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Size of promoted object
    #[serde(rename = "objectSize")]
    pub object_size: u64,
    ///Tenuring age of a surviving object before being copied. The tenuring age of an object is a value between 0-15 and is incremented each scavange the object survives. Newly allocated objects have tenuring age 0.
    #[serde(rename = "tenuringAge")]
    pub tenuring_age: u32,
    ///True if object was promoted to Old space, otherwise the object was aged and copied to a Survivor space
    #[serde(rename = "tenured")]
    pub tenured: bool,
    ///Size of the allocated PLAB to which the object was copied
    #[serde(rename = "plabSize")]
    pub plab_size: u64,
}
///Object survived scavenge and was copied directly to the heap. Supported GCs are Parallel Scavange, G1 and CMS with Parallel New. Due to promotion being done in parallel an object might be reported multiple times as the GC threads race to copy all objects.
#[derive(Clone, Debug, Deserialize)]
pub struct PromoteObjectOutsidePLAB {
    ///Identifier signifying GC during which the object was promoted
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Class of promoted object
    #[serde(rename = "objectClass")]
    pub object_class: Option<Class>,
    ///Size of promoted object
    #[serde(rename = "objectSize")]
    pub object_size: u64,
    ///Tenuring age of a surviving object before being copied. The tenuring age of an object is a value between 0-15 and is incremented each scavange the object survives. Newly allocated objects have tenuring age 0.
    #[serde(rename = "tenuringAge")]
    pub tenuring_age: u32,
    ///True if object was promoted to Old space, otherwise the object was aged and copied to a Survivor space
    #[serde(rename = "tenured")]
    pub tenured: bool,
}
///Promotion of an object failed
#[derive(Clone, Debug, Deserialize)]
pub struct PromotionFailed {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Promotion Failed Data
    #[serde(rename = "promotionFailed")]
    pub promotion_failed: Option<CopyFailed>,
    ///Running thread
    #[serde(rename = "thread")]
    pub thread: Option<Thread>,
}
///ProtectionDomainCacheTableStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct ProtectionDomainCacheTableStatistics {
    ///Number of buckets
    #[serde(rename = "bucketCount")]
    pub bucket_count: u64,
    ///Number of all entries
    #[serde(rename = "entryCount")]
    pub entry_count: u64,
    ///Total memory footprint (the table itself plus all of the entries)
    #[serde(rename = "totalFootprint")]
    pub total_footprint: u64,
    ///The maximum bucket length (entries in a single bucket)
    #[serde(rename = "bucketCountMaximum")]
    pub bucket_count_maximum: u64,
    ///The average bucket length
    #[serde(rename = "bucketCountAverage")]
    pub bucket_count_average: f32,
    ///How far bucket lengths are spread out from their average value
    #[serde(rename = "bucketCountVariance")]
    pub bucket_count_variance: f32,
    ///How far bucket lengths are spread out from their mean (expected) value
    #[serde(rename = "bucketCountStandardDeviation")]
    pub bucket_count_standard_deviation: f32,
    ///How many items were added since last event (per second)
    #[serde(rename = "insertionRate")]
    pub insertion_rate: f32,
    ///How many items were removed since last event (per second)
    #[serde(rename = "removalRate")]
    pub removal_rate: f32,
}
///RedefineClasses
#[derive(Clone, Debug, Deserialize)]
pub struct RedefineClasses {
    ///Class Count
    #[serde(rename = "classCount")]
    pub class_count: i32,
    ///Class Redefinition Id
    #[serde(rename = "redefinitionId")]
    pub redefinition_id: u64,
}
///Activation of Reserved Stack Area caused by stack overflow with ReservedStackAccess annotated method in call stack
#[derive(Clone, Debug, Deserialize)]
pub struct ReservedStackActivation {
    ///Java Method
    #[serde(rename = "method")]
    pub method: Option<Method>,
}
///RetransformClasses
#[derive(Clone, Debug, Deserialize)]
pub struct RetransformClasses {
    ///Class Count
    #[serde(rename = "classCount")]
    pub class_count: i32,
    ///Class Redefinition Id
    #[serde(rename = "redefinitionId")]
    pub redefinition_id: u64,
}
///Safepointing begin
#[derive(Clone, Debug, Deserialize)]
pub struct SafepointBegin {
    ///Safepoint Identifier
    #[serde(rename = "safepointId")]
    pub safepoint_id: u64,
    ///The total number of threads at the start of safe point
    #[serde(rename = "totalThreadCount")]
    pub total_thread_count: i32,
    ///The number of threads in JNI critical sections
    #[serde(rename = "jniCriticalThreadCount")]
    pub jni_critical_thread_count: i32,
}
///Safepointing begin running cleanup tasks
#[derive(Clone, Debug, Deserialize)]
pub struct SafepointCleanup {
    ///Safepoint Identifier
    #[serde(rename = "safepointId")]
    pub safepoint_id: u64,
}
///Safepointing begin running cleanup tasks
#[derive(Clone, Debug, Deserialize)]
pub struct SafepointCleanupTask {
    ///Safepoint Identifier
    #[serde(rename = "safepointId")]
    pub safepoint_id: u64,
    ///The task name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///Safepointing end
#[derive(Clone, Debug, Deserialize)]
pub struct SafepointEnd {
    ///Safepoint Identifier
    #[serde(rename = "safepointId")]
    pub safepoint_id: u64,
}
///Synchronize run state of threads
#[derive(Clone, Debug, Deserialize)]
pub struct SafepointStateSynchronization {
    ///Safepoint Identifier
    #[serde(rename = "safepointId")]
    pub safepoint_id: u64,
    ///The number of threads running at the beginning of state check
    #[serde(rename = "initialThreadCount")]
    pub initial_thread_count: i32,
    ///The number of threads still running
    #[serde(rename = "runningThreadCount")]
    pub running_thread_count: i32,
    ///Number of state check iterations
    #[serde(rename = "iterations")]
    pub iterations: i32,
}
///Information about a specific heap region in the Shenandoah GC
#[derive(Clone, Debug, Deserialize)]
pub struct ShenandoahHeapRegionInformation {
    ///Index
    #[serde(rename = "index")]
    pub index: u32,
    ///State
    #[serde(rename = "state")]
    pub state: Option<ShenandoahHeapRegionState>,
    ///Start
    #[serde(rename = "start")]
    pub start: u64,
    ///Used
    #[serde(rename = "used")]
    pub used: u64,
}
///Information about a Shenandoah heap region state change
#[derive(Clone, Debug, Deserialize)]
pub struct ShenandoahHeapRegionStateChange {
    ///Index
    #[serde(rename = "index")]
    pub index: u32,
    ///From
    #[serde(rename = "from")]
    pub from: Option<ShenandoahHeapRegionState>,
    ///To
    #[serde(rename = "to")]
    pub to: Option<ShenandoahHeapRegionState>,
    ///Start
    #[serde(rename = "start")]
    pub start: u64,
    ///Used
    #[serde(rename = "used")]
    pub used: u64,
}
///JVM shutting down
#[derive(Clone, Debug, Deserialize)]
pub struct Shutdown {
    ///Reason for JVM shutdown
    #[serde(rename = "reason")]
    pub reason: Option<String>,
}
///StringFlag
#[derive(Clone, Debug, Deserialize)]
pub struct StringFlag {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: Option<String>,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///StringFlagChanged
#[derive(Clone, Debug, Deserialize)]
pub struct StringFlagChanged {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Old Value
    #[serde(rename = "oldValue")]
    pub old_value: Option<String>,
    ///New Value
    #[serde(rename = "newValue")]
    pub new_value: Option<String>,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///StringTableStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct StringTableStatistics {
    ///Number of buckets
    #[serde(rename = "bucketCount")]
    pub bucket_count: u64,
    ///Number of all entries
    #[serde(rename = "entryCount")]
    pub entry_count: u64,
    ///Total memory footprint (the table itself plus all of the entries)
    #[serde(rename = "totalFootprint")]
    pub total_footprint: u64,
    ///The maximum bucket length (entries in a single bucket)
    #[serde(rename = "bucketCountMaximum")]
    pub bucket_count_maximum: u64,
    ///The average bucket length
    #[serde(rename = "bucketCountAverage")]
    pub bucket_count_average: f32,
    ///How far bucket lengths are spread out from their average value
    #[serde(rename = "bucketCountVariance")]
    pub bucket_count_variance: f32,
    ///How far bucket lengths are spread out from their mean (expected) value
    #[serde(rename = "bucketCountStandardDeviation")]
    pub bucket_count_standard_deviation: f32,
    ///How many items were added since last event (per second)
    #[serde(rename = "insertionRate")]
    pub insertion_rate: f32,
    ///How many items were removed since last event (per second)
    #[serde(rename = "removalRate")]
    pub removal_rate: f32,
}
///SweepCodeCache
#[derive(Clone, Debug, Deserialize)]
pub struct SweepCodeCache {
    ///Sweep Identifier
    #[serde(rename = "sweepId")]
    pub sweep_id: i32,
    ///Methods Swept
    #[serde(rename = "sweptCount")]
    pub swept_count: u32,
    ///Methods Flushed
    #[serde(rename = "flushedCount")]
    pub flushed_count: u32,
    ///Methods Zombified
    #[serde(rename = "zombifiedCount")]
    pub zombified_count: u32,
}
///SymbolTableStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct SymbolTableStatistics {
    ///Number of buckets
    #[serde(rename = "bucketCount")]
    pub bucket_count: u64,
    ///Number of all entries
    #[serde(rename = "entryCount")]
    pub entry_count: u64,
    ///Total memory footprint (the table itself plus all of the entries)
    #[serde(rename = "totalFootprint")]
    pub total_footprint: u64,
    ///The maximum bucket length (entries in a single bucket)
    #[serde(rename = "bucketCountMaximum")]
    pub bucket_count_maximum: u64,
    ///The average bucket length
    #[serde(rename = "bucketCountAverage")]
    pub bucket_count_average: f32,
    ///How far bucket lengths are spread out from their average value
    #[serde(rename = "bucketCountVariance")]
    pub bucket_count_variance: f32,
    ///How far bucket lengths are spread out from their mean (expected) value
    #[serde(rename = "bucketCountStandardDeviation")]
    pub bucket_count_standard_deviation: f32,
    ///How many items were added since last event (per second)
    #[serde(rename = "insertionRate")]
    pub insertion_rate: f32,
    ///How many items were removed since last event (per second)
    #[serde(rename = "removalRate")]
    pub removal_rate: f32,
}
///SyncOnValueBasedClass
#[derive(Clone, Debug, Deserialize)]
pub struct SyncOnValueBasedClass {
    ///Value Based Class
    #[serde(rename = "valueBasedClass")]
    pub value_based_class: Option<Class>,
}
///SystemGC
#[derive(Clone, Debug, Deserialize)]
pub struct SystemGC {
    ///Invoked Concurrent
    #[serde(rename = "invokedConcurrent")]
    pub invoked_concurrent: bool,
}
///SystemProcess
#[derive(Clone, Debug, Deserialize)]
pub struct SystemProcess {
    ///Process Identifier
    #[serde(rename = "pid")]
    pub pid: Option<String>,
    ///Command Line
    #[serde(rename = "commandLine")]
    pub command_line: Option<String>,
}
///TenuringDistribution
#[derive(Clone, Debug, Deserialize)]
pub struct TenuringDistribution {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Age
    #[serde(rename = "age")]
    pub age: u32,
    ///Size
    #[serde(rename = "size")]
    pub size: u64,
}
///ThreadAllocationStatistics
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadAllocationStatistics {
    ///Approximate number of bytes allocated since thread start
    #[serde(rename = "allocated")]
    pub allocated: u64,
    ///Thread
    #[serde(rename = "thread")]
    pub thread: Option<Thread>,
}
///ThreadCPULoad
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadCPULoad {
    ///User mode thread CPU load
    #[serde(rename = "user")]
    pub user: f32,
    ///System mode thread CPU load
    #[serde(rename = "system")]
    pub system: f32,
}
///ThreadContextSwitchRate
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadContextSwitchRate {
    ///Number of context switches per second
    #[serde(rename = "switchRate")]
    pub switch_rate: f32,
}
///ThreadDump
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadDump {
    ///Thread Dump
    #[serde(rename = "result")]
    pub result: Option<String>,
}
///ThreadEnd
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadEnd {
    ///Java Thread
    #[serde(rename = "thread")]
    pub thread: Option<Thread>,
}
///ThreadPark
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadPark {
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
    pub address: u64,
}
///ThreadSleep
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadSleep {
    ///Sleep Time
    #[serde(rename = "time")]
    pub time: i64,
}
///ThreadStart
#[derive(Clone, Debug, Deserialize)]
pub struct ThreadStart {
    ///New Java Thread
    #[serde(rename = "thread")]
    pub thread: Option<Thread>,
    ///Parent Java Thread
    #[serde(rename = "parentThread")]
    pub parent_thread: Option<Thread>,
}
///UnsignedIntFlag
#[derive(Clone, Debug, Deserialize)]
pub struct UnsignedIntFlag {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: u32,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///UnsignedIntFlagChanged
#[derive(Clone, Debug, Deserialize)]
pub struct UnsignedIntFlagChanged {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Old Value
    #[serde(rename = "oldValue")]
    pub old_value: u32,
    ///New Value
    #[serde(rename = "newValue")]
    pub new_value: u32,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///UnsignedLongFlag
#[derive(Clone, Debug, Deserialize)]
pub struct UnsignedLongFlag {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Value
    #[serde(rename = "value")]
    pub value: u64,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///UnsignedLongFlagChanged
#[derive(Clone, Debug, Deserialize)]
pub struct UnsignedLongFlagChanged {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
    ///Old Value
    #[serde(rename = "oldValue")]
    pub old_value: u64,
    ///New Value
    #[serde(rename = "newValue")]
    pub new_value: u64,
    ///Origin
    #[serde(rename = "origin")]
    pub origin: Option<FlagValueOrigin>,
}
///Description of the virtualization technology the JVM runs on
#[derive(Clone, Debug, Deserialize)]
pub struct VirtualizationInformation {
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///Extra information specific to Young Garbage Collections
#[derive(Clone, Debug, Deserialize)]
pub struct YoungGarbageCollection {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Tenuring Threshold
    #[serde(rename = "tenuringThreshold")]
    pub tenuring_threshold: u32,
}
///The configuration of the young generation of the garbage collected heap
#[derive(Clone, Debug, Deserialize)]
pub struct YoungGenerationConfiguration {
    ///Minimum Young Generation Size
    #[serde(rename = "minSize")]
    pub min_size: u64,
    ///Maximum Young Generation Size
    #[serde(rename = "maxSize")]
    pub max_size: u64,
    ///The size of the young generation relative to the tenured generation
    #[serde(rename = "newRatio")]
    pub new_ratio: u32,
}
///Time spent waiting for memory to become available
#[derive(Clone, Debug, Deserialize)]
pub struct ZAllocationStall {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<ZPageTypeType>,
    ///Size
    #[serde(rename = "size")]
    pub size: u64,
}
///Allocation of a ZPage
#[derive(Clone, Debug, Deserialize)]
pub struct ZPageAllocation {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<ZPageTypeType>,
    ///Size
    #[serde(rename = "size")]
    pub size: u64,
    ///Flushed
    #[serde(rename = "flushed")]
    pub flushed: u64,
    ///Committed
    #[serde(rename = "committed")]
    pub committed: u64,
    ///Segments
    #[serde(rename = "segments")]
    pub segments: u32,
    ///Non-blocking
    #[serde(rename = "nonBlocking")]
    pub non_blocking: bool,
}
///ZRelocationSet
#[derive(Clone, Debug, Deserialize)]
pub struct ZRelocationSet {
    ///Total
    #[serde(rename = "total")]
    pub total: u64,
    ///Empty
    #[serde(rename = "empty")]
    pub empty: u64,
    ///Relocate
    #[serde(rename = "relocate")]
    pub relocate: u64,
}
///ZRelocationSetGroup
#[derive(Clone, Debug, Deserialize)]
pub struct ZRelocationSetGroup {
    ///Type
    #[serde(rename = "type")]
    pub r#type: Option<ZPageTypeType>,
    ///Pages
    #[serde(rename = "pages")]
    pub pages: u64,
    ///Total
    #[serde(rename = "total")]
    pub total: u64,
    ///Empty
    #[serde(rename = "empty")]
    pub empty: u64,
    ///Relocate
    #[serde(rename = "relocate")]
    pub relocate: u64,
}
///ZStatisticsCounter
#[derive(Clone, Debug, Deserialize)]
pub struct ZStatisticsCounter {
    ///Id
    #[serde(rename = "id")]
    pub id: Option<ZStatisticsCounterType>,
    ///Increment
    #[serde(rename = "increment")]
    pub increment: u64,
    ///Value
    #[serde(rename = "value")]
    pub value: u64,
}
///ZStatisticsSampler
#[derive(Clone, Debug, Deserialize)]
pub struct ZStatisticsSampler {
    ///Id
    #[serde(rename = "id")]
    pub id: Option<ZStatisticsSamplerType>,
    ///Value
    #[serde(rename = "value")]
    pub value: u64,
}
///ZThreadPhase
#[derive(Clone, Debug, Deserialize)]
pub struct ZThreadPhase {
    ///GC Identifier
    #[serde(rename = "gcId")]
    pub gc_id: u32,
    ///Name
    #[serde(rename = "name")]
    pub name: Option<String>,
}
///Uncommitting of memory
#[derive(Clone, Debug, Deserialize)]
pub struct ZUncommit {
    ///Uncommitted
    #[serde(rename = "uncommitted")]
    pub uncommitted: u64,
}
///Unmapping of memory
#[derive(Clone, Debug, Deserialize)]
pub struct ZUnmap {
    ///Unmapped
    #[serde(rename = "unmapped")]
    pub unmapped: u64,
}
