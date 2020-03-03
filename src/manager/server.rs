use super::channel::Channel;
use super::client::ExtensionManagerClient;
use super::Plugin;

use crate::gen::osquery;

use std::collections::BTreeMap;
pub struct ExtensionManagerServer<C: Channel> {
    name: String,
    client: ExtensionManagerClient<C>,
    registry: super::PluginRegistry,
}

impl<C> ExtensionManagerServer<C>
where
    C: Channel,
{
    pub fn new_from_channel(name: &str, channel: C) -> Self {
        let client = ExtensionManagerClient::new_with_channel(channel);
        Self {
            name: name.into(),
            client: client,
            registry: Self::new_registry(),
        }
    }

    // pub fn new<P: AsRef<Path>>new(path: P) -> Self {

    // }

    /// Register plugins with the server
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        let plugin_reg_name = plugin.registry_name();
        if self.registry.contains_key(&plugin_reg_name) {
            // get the appropriate sub-registry and insert the plugin by name
            self.registry
                .get_mut(&plugin_reg_name)
                .and_then(|s| Some(s.insert(plugin.name().into(), plugin)))
                .map(|_| unreachable!())
                .unwrap();
        }
        panic!("Unsupported plugin type: {:?}", plugin_reg_name);
    }

    /// Call this to start related plugins
    pub fn run(self) {
        // build registry
        let registry = self.build_registry();

        // create client
        // register plugin registry with via client
        // setup handler and router/processor

        // let handler = crate::manager::client::ExtensionManagerHandler::new(registry);
        // let processor = osquery::ExtensionSyncProcessor::new();

        // wait in another thread for pings from osquery, send signal to stop if no heartbeat

        unimplemented!()
    }

    /// Create new registry containing sub-registries for valid plugin classes
    fn new_registry() -> super::PluginRegistry {
        let mut registry = BTreeMap::new();
        use strum::IntoEnumIterator;
        for variant in crate::plugin::PluginVariant::iter() {
            registry.insert(variant.into(), BTreeMap::new());
        }
        registry
    }

    fn build_registry(&self) -> osquery::ExtensionRegistry {
        let mut registry = osquery::ExtensionRegistry::new();
        for (plugin_class, plugins) in &self.registry {
            let mut route_map = BTreeMap::new();
            // build route map
            for (plugin_class, plugin) in plugins {
                route_map.insert(plugin_class.into(), plugin.routes());
            }
            registry.insert(plugin_class.to_string(), route_map);
        }
        registry
    }
}
