# rust_plugins_example

This is a quick and dirty FFI Rust plugins example.

## plugin_demo

This is the plugin library, which exports the necessary `Plugin` trait and `declare_plugin!` macro.

This crate also includes a binary, `plugin_demo` which can be run to actually demonstrate loading plugins with the system.

*By default, plugins would be loaded from the `plugins` directory in the root.*

## demo_plug

This is the actual plugin used to demonstrate loading.

Building this will provide a dynamic library file in `target/{debug|release}` that can be copied to the plugin directory.