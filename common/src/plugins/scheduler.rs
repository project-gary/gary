
///A Trait for all scheduler plugins.
pub trait SchedulerPlugin: Any + Send + Sync {
    /// The name of the plugin used to identify it.
    fn name(&self) -> String;
    /// A callback fired immediately after the plugin is loaded. Usually used
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    
}
