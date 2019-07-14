use std::any::Any;
use std::fmt;
/*
 * Experimental
 * consider the use of this atfilename: Pfilename: P your own risk, eventually this will
 * be versioned and safe to use, currently it may change at whim.
 * Use of this api now may mean a lot of work keeping up with it.
*/

/// A trait for plugins that run workloads
/// Any unimplemented function should return RuntimeErrors::Unimplemented
/// as this is used internally to ensure functionality of the plugin.
pub trait RuntimePlugin: Any + Send + Sync {
    //Required for all plugins

    /// The name of the plugin used to identify it.
    fn name(&self) -> String;
    /// A callback fired immediately after the plugin is loaded. Usually used
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    fn get_features(&self) -> Vec<RuntimeFeatures>;
    fn get_version(&self) -> i32;

    //Required for feature WorkloadRunner
    fn create_workload(
        &self,
        id: String,
        config: &RuntimeConfig,
        options: &Option<SandboxConfig>,
    ) -> Result<String, RuntimeError>;

    fn start_workload(&mut self, id: String) -> Option<RuntimeError>;
    fn stop_workload(&self, id: String, timeout: i32) -> Option<RuntimeError>;
    fn remove_workload(&self, id: String) -> Option<RuntimeError>;
    fn status_workload(&self, id: String) -> Result<WorkloadStatus, RuntimeError>;

    //Required for feature container && vm
    fn update_workload_resources(&self, id: String, rez: WorkloadResources)
        -> Option<RuntimeError>;
    fn exec_sync(
        &self,
        id: String,
        cmd: &[String],
        timeout: i32,
    ) -> (&[u8], &[u8], Option<RuntimeError>);

    //Required for feature vm

    //Required for feature function
}

pub struct WorkloadResources {}

pub struct WorkloadStatus {}

#[derive(Debug, PartialEq)]
pub enum RuntimeFeatures {
    WorkloadRunner,
    Container,
    VM,
    Function,
}

pub struct RuntimeConfig {}

pub struct SandboxConfig {}

#[derive(Debug)]
pub enum RuntimeErrorType {
    Unimplemented,
    Timeout,
    Unknown,
}

#[derive(Debug)]
pub struct RuntimeError {
    error_type: RuntimeErrorType,
}

impl std::error::Error for RuntimeError {
    fn description(&self) -> &str {
        "This action has failed."
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError has arrived.")
    }
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorType) -> Self {
        return RuntimeError { error_type: kind };
    }
}
