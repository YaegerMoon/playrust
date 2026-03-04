pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
    pub plugins: Vec<Box<dyn Plugin>>,
}

// 3. Implement the PluginManager
impl PluginManager {
    pub fn new() -> PluginManager {
        return PluginManager {
            plugins: Vec::new(),
        };
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let reuslt = self
            .plugins
            .iter()
            .find(|item| item.name() == plugin.name());

        match reuslt {
            Some(_) => {
                panic!("Duplicated Plugin")
            }
            None => {
                self.plugins.push(plugin);
            }
        }
    }

    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        if let Some(pos) = self.plugins.iter().position(|p| p.name() == name) {
            Some(self.plugins.remove(pos))
        } else {
            None
        }
    }

    pub fn execute_all(&self) {
        self.plugins.iter().for_each(|p| p.execute());
    }
}

// Example usage
pub struct MyPlugin {
    name: String,
}

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    fn execute(&self) {
        println!("Executing {}", self.name);
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self {
            name: "myplugin".to_string(),
        }
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
    manager.remove_plugin("myplugin");
    manager.execute_all();
}
