use std::fmt;

/*
 * Experimental
 * consider the use of this at your own risk, eventually this will
 * be versioned and safe to use, currently it may change at whim.
 * Use of this api now may mean a lot of work keeping up with it.
*/

pub trait Runtime {
    //Required for all plugins
    fn get_features() -> Vec<RuntimeFeatures>;
    fn get_version() -> i32;
    fn get_plugin_name() -> String;

    //Required for feature WorkloadRunner
    fn create_workload(
        id: String,
        config: &RuntimeConfig,
        options: &Option<SandboxConfig>,
    ) -> Result<String, RuntimeError>;

    fn start_workload(id: String) -> Option<RuntimeError>;
    fn stop_workload(id: String, timeout: i32) -> Option<RuntimeError>;
    fn remove_workload(id: String) -> Option<RuntimeError>;
    fn status_workload(id: String) -> Result<WorkloadStatus, RuntimeError>;

    //Required for feature container && vm
    fn update_workload_resources(id: String, rez: WorkloadResources) -> Option<RuntimeError>;
    fn exec_sync(id: String, cmd: &[String], timeout: i32) -> (&[u8], &[u8], Option<RuntimeError>);

    //Required for feature vm

    //Required for feature function
}

pub struct WorkloadResources {}

pub struct WorkloadStatus {}

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
    pub fn new(kind: RuntimeErrorType) -> RuntimeError {
        return RuntimeError { error_type: kind };
    }
}
