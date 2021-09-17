use std::any::Any;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use libloading::{Library, Symbol};

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

pub mod ffi;

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn on_plugin_load(&self) -> Result<(), String>;
    fn on_plugin_unload(&self) -> Result<(), String>;
}

pub struct PluginManager {
    plugin_directory: OsString,
    plugins: Arc<Mutex<HashMap<String, Arc<Box<dyn Plugin>>>>>,
    libraries: Arc<Mutex<HashMap<String, Library>>>,
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        self.unload();
    }
}

impl PluginManager {
    pub unsafe fn from_raw(path: *mut OsString) -> Self {
        Self {
            plugin_directory: <*const OsString>::as_ref(path).unwrap().clone(),
            plugins: Arc::new(Mutex::new(HashMap::new())),
            libraries: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn new(path: &str) -> Self {
        Self {
            plugin_directory: OsString::from(path),
            plugins: Arc::new(Mutex::new(HashMap::new())),
            libraries: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn get(&self, name: String) -> Option<Arc<Box<dyn Plugin>>> {
        let lock = self.plugins.lock().unwrap();
        if let Some(plugin) = (*lock).get(&name) {
            Some(plugin.clone())
        } else {
            None
        }
    }
    pub unsafe fn load_plugins(&self) -> Result<(), String> {
        let library_path = self.plugin_directory.clone();
        for p in fs::read_dir(&library_path).unwrap() {
            if let Ok(entry) = p {
                let entry_path = entry.path();
                let meta = fs::metadata(entry_path.clone()).unwrap();
                if meta.is_file() {
                    self.load_plugin(entry_path)?;
                }
            }
        }
        Ok(())
    }
    pub fn unload(&self) {
        {
            let plugins = self.plugins.lock().unwrap();
            for plugin in (*plugins).values() {
                plugin.on_plugin_unload().unwrap();
            }
        }
        {
            let mut libs = self.libraries.lock().unwrap();
            for (_, lib) in (*libs).drain() {
                drop(lib);
            }
        }
    }
    unsafe fn load_plugin<P>(&self, filepath: P) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        let lib = Library::new(filepath.as_ref())
            .map_err(|err| format!("unable to load plugin: {}", err))?;
        let mut libs = self.libraries.lock().unwrap();
        let mut plugins = self.plugins.lock().unwrap();
        let constructor = lib
            .get::<Symbol<unsafe fn() -> *mut dyn Plugin>>(b"_plugin_create")
            .map_err(|err| format!("the '_plugin_create' symbol was not found: {}", err))?;
        let boxed_raw = constructor();
        let plugin = Arc::new(Box::from_raw(boxed_raw));
        #[cfg(debug_assertions)]
        plugin
            .on_plugin_load()
            .map_err(|e| format!("plugin load error: {}", e))?;
        let plugin_name = plugin.name().to_string();
        println!("Loaded plugin: {}", plugin_name);
        (*plugins).insert(plugin_name.clone(), plugin);
        (*libs).insert(plugin_name.clone(), lib);
        Ok(())
    }
    pub fn has_plugin(&self, name: String) -> bool {
        let lock = self.plugins.lock().unwrap();
        (*lock).contains_key(&name)
    }
}
