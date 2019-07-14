pub mod runtime;

use runtime::*;

use libloading::{Library, Symbol};
use std::env;
use std::ffi::OsStr;

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

    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<(), ()> {
        type PluginCreate = unsafe fn() -> *mut RuntimePlugin;

        let mut path = env::current_dir().unwrap();
        path.push("plugins/");
        path.push(filename.as_ref());

        let lib = Library::new(path.into_os_string()).unwrap();

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
    fn create_workload_by_name(
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
    fn create_workload_by_type(
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

    fn start_workload(&self, workload_id: String, plugin_name: String) -> Option<RuntimeError> {
        for plugin in &self.plugins {
            if (plugin.name() == plugin_name) {
                return plugin.start_workload(workload_id);
            }
        }

        return Some(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    fn stop_workload(
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

    fn remove_workload(&self, workload_id: String, plugin_name: String) -> Option<RuntimeError> {
        for plugin in &self.plugins {
            if (plugin.name() == plugin_name) {
                return plugin.remove_workload(workload_id);
            }
        }

        return Some(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    fn status_workload(
        &self,
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
    fn update_workload_resources(
        &self,
        id: String,
        rez: WorkloadResources,
    ) -> Option<RuntimeError> {
        return None;
    }

    fn exec_sync(
        &self,
        id: String,
        cmd: &[String],
        timeout: i32,
    ) -> (&[u8], &[u8], Option<RuntimeError>) {
        return (&[0], &[0], None);
    }
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut $crate::plugins::runtime::RuntimePlugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$crate::plugins::runtime::RuntimePlugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
