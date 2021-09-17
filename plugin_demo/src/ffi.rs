use std::{ffi::OsString, os::raw::c_int};

use crate::PluginManager;

#[no_mangle]
pub unsafe extern "C" fn plugin_manager_new(plugin_directory: *mut OsString) -> *mut PluginManager {
    Box::into_raw(Box::new(PluginManager::from_raw(plugin_directory)))
}

#[no_mangle]
pub unsafe extern "C" fn plugin_manager_destroy(pm: *mut PluginManager) {
    if !pm.is_null() {
        let pm = Box::from_raw(pm);
        drop(pm);
    }
}

#[no_mangle]
pub unsafe extern "C" fn plugin_manager_load_plugins(pm: *mut PluginManager) -> c_int {
    let pm = &*pm;
    if let Ok(()) = pm.load_plugins() {
        0
    } else {
        1
    }
}
