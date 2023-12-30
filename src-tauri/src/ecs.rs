use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;

trait Plugin: Send + Sync {
    fn run(&self);
}

#[derive(Debug)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn run(&self) {
        println!("Running {:?}", Self);
    }
}

struct PluginManager {
    plugins: RwLock<HashMap<String, Arc<dyn Plugin>>>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: RwLock::new(HashMap::new()),
        }
    }

    fn register_plugin(&self, name: &str, plugin: Arc<dyn Plugin>) {
        let mut plugins = self.plugins.write().unwrap();
        plugins.insert(name.to_string(), plugin);
    }

    fn run_all_plugins(&self) {
        let plugins = self.plugins.read().unwrap();
        for (_, plugin) in plugins.iter() {
            plugin.run();
        }
    }
}

static PLUGIN_MANAGER: Lazy<Arc<PluginManager>> = Lazy::new(|| Arc::new(PluginManager::new()));

fn register_global_plugin(name: &str, plugin: Arc<dyn Plugin>) {
    PLUGIN_MANAGER.register_plugin(name, plugin);
}

fn run_global_plugins() {
    PLUGIN_MANAGER.run_all_plugins();
}

fn main() {
    let my_plugin = Arc::new(MyPlugin);
    register_global_plugin("MyPlugin", my_plugin.clone());
    run_global_plugins();
    test();
}

fn test() {
    run_global_plugins()
}