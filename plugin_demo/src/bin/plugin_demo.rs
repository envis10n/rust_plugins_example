use plugin_demo::PluginManager;

fn main() {
    let plugin_manager = PluginManager::new("plugins/");
    println!("Loading plugins...");
    unsafe {
        plugin_manager.load_plugins().unwrap();
    }
    println!("Done.");
}
