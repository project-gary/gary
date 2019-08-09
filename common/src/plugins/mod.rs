pub mod runtime;

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
