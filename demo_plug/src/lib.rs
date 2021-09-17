use plugin_demo::{Plugin, declare_plugin};

#[derive(Debug, Default)]
pub struct DemoPlug;

impl Plugin for DemoPlug {
    fn name(&self) -> &'static str {
        "Demo Plug"
    }
    fn on_plugin_load(&self) -> Result<(), String> {
        println!("Demo Plug On Load");
        Ok(())
    }
    fn on_plugin_unload(&self) -> Result<(), String> {
        println!("Demo Plug Unload");
        Ok(())
    }
}

declare_plugin!(DemoPlug, DemoPlug::default);
