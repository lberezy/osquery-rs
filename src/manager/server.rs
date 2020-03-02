use super::channel::Channel;
use super::client::Client;
use super::Plugin;

use crate::gen::osquery;

use std::collections::BTreeMap;
pub struct ExtensionServer<C: Channel> {
    name: String,
    client: Client<C>,
    registry: super::PluginRegistry,
}

impl<C> ExtensionServer<C>
where
    C: Channel,
{
    pub fn new_from_channel(name: &str, channel: C) -> Self {
        // unimplemented!()
        let client = Client::new_with_channel(channel);
        Self {
            name: name.into(),
            client: client,
            registry: Self::new_registry(),
        }
    }
    /// Register plugins with the server
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        let plugin_reg_name = plugin.registry_name();
        if self.registry.contains_key(plugin_reg_name) {
            // get the appropriate sub-registry and insert the plugin by name
            self.registry
                .get_mut(plugin_reg_name)
                .and_then(|s| Some(s.insert(plugin.name().into(), plugin)))
                .map(|_| unreachable!())
                .unwrap();
        }
        panic!("Unsupported plugin type: {:?}", plugin_reg_name);
    }

    /// Call this to start related plugins
    pub fn run(self) {}

    /// Create new registry containing sub-registries for valid plugin types
    fn new_registry() -> super::PluginRegistry {
        let mut registry = BTreeMap::new();
        registry.insert(crate::plugin::PluginVariant::Table.into(), BTreeMap::new());
        registry
    }
}
