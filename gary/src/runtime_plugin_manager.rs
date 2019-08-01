use core::plugins::runtime::*;

use libloading::{Library, Symbol};
use std::env;
use std::ffi::OsStr;

use std::fs;
use std::path::PathBuf;
pub struct RuntimePluginManager {
    //TODO: could be a Map<Name,Plugin>, might be useful to be able to access them by name
    plugins: Vec<Box<RuntimePlugin>>,
    loaded_libraries: Vec<Library>,
}

impl RuntimePluginManager {
    pub fn new() -> RuntimePluginManager {
        RuntimePluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub fn load_plugins_in_dir(&mut self, dir: String) -> Result<(), ()> {
        let path = PathBuf::from(dir);
        if let Ok(files) = fs::read_dir(path) {
            for f in files {
                if let Ok(f) = f {
                    unsafe {
                        self.load_plugin(f.path());
                    }
                }
            }
        }

        Ok(())
    }

    pub fn load_in_memory_plugin(&mut self, plugin: Box<RuntimePlugin>) {
        self.plugins.push(plugin);
    }

    pub unsafe fn load_plugin(&mut self, file: PathBuf) -> Result<(), ()> {
        type PluginCreate = unsafe fn() -> *mut RuntimePlugin;

        let lib = Library::new(file.into_os_string()).unwrap();

        // We need to keep the library around otherwise our plugin's vtable will
        // point to garbage. We do this little dance to make sure the library
        // doesn't end up getting moved.
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create").unwrap();
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);

        // Call `on_plugin_load` to let the plugin know we have connected
        plugin.on_plugin_load();
        self.plugins.push(plugin);

        Ok(())
    }

    /// Not useful... as you have to know it's name... to get it's name.
    pub fn get_plugin_name(&self, id: String) -> Result<String, RuntimeError> {
        for plugin in &self.plugins {
            if plugin.name() == id {
                return Ok(plugin.name());
            }
        }
        Err(RuntimeError::new(RuntimeErrorType::Unknown))
    }

    /// returns ID of created workload
    pub fn create_workload_by_name(
        &self,
        plugin_name: String,
        sandbox_id: String,
        config: &RuntimeConfig,
        options: &Option<SandboxConfig>,
    ) -> Result<String, RuntimeError> {
        for plugin in &self.plugins {
            if plugin.name() == plugin_name {
                return plugin.create_workload(sandbox_id, config, options);
            }
        }
        Err(RuntimeError::new(RuntimeErrorType::Unknown))
    }

    /// Required for feature WorkloadRunner
    /// returns (workloadID, pluginName)
    /// TODO: reduce complexity/nesting with map
    pub fn create_workload_by_type(
        &self,
        plugin_type: RuntimeFeatures,
        sandbox_id: String,
        config: &RuntimeConfig,
        options: &Option<SandboxConfig>,
    ) -> Result<(String, String), RuntimeError> {
        for plugin in &self.plugins {
            for feature in plugin.get_features() {
                if feature == plugin_type {
                    let results = plugin.create_workload(sandbox_id, config, options);
                    match results {
                        Ok(r) => return Ok((plugin.name(), r)),
                        _ => return Err(RuntimeError::new(RuntimeErrorType::Unknown)),
                    }
                }
            }
        }
        Err(RuntimeError::new(RuntimeErrorType::Unknown))
    }

    pub fn start_workload(
        &mut self,
        workload_id: String,
        plugin_name: String,
    ) -> Option<RuntimeError> {
        for plugin in &mut self.plugins {
            if (plugin.name() == plugin_name) {
                return plugin.start_workload(workload_id);
            }
        }

        return Some(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    pub fn stop_workload(
        &self,
        workload_id: String,
        plugin_name: String,
        timeout: i32,
    ) -> Option<RuntimeError> {
        for plugin in &self.plugins {
            if (plugin.name() == plugin_name) {
                return plugin.stop_workload(workload_id, timeout);
            }
        }

        return Some(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    pub fn remove_workload(
        &self,
        workload_id: String,
        plugin_name: String,
    ) -> Option<RuntimeError> {
        for plugin in &self.plugins {
            if (plugin.name() == plugin_name) {
                return plugin.remove_workload(workload_id);
            }
        }

        return Some(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    pub fn status_workload(
        &mut self,
        workload_id: String,
        plugin_name: String,
    ) -> Result<WorkloadStatus, RuntimeError> {
        for plugin in &self.plugins {
            if (plugin.name() == plugin_name) {
                return plugin.status_workload(workload_id);
            }
        }

        return Err(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    //Required for feature container && vm
    pub fn update_workload_resources(
        &self,
        id: String,
        rez: WorkloadResources,
    ) -> Option<RuntimeError> {
        return None;
    }

    pub fn exec_sync(
        &self,
        id: String,
        cmd: &[String],
        timeout: i32,
    ) -> (&[u8], &[u8], Option<RuntimeError>) {
        return (&[0], &[0], None);
    }
}
