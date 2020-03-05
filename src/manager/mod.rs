mod async_server;
mod client;
mod comms;
mod server;

use crate::plugin::{Plugin, PluginVariant};
use std::collections::BTreeMap;

pub type PluginRegistry = BTreeMap<PluginVariant, BTreeMap<String, Box<dyn Plugin>>>;
use crate::gen::osquery;
pub struct ManagerPluginRegistry {
    pub registry: PluginRegistry,
}

impl From<ManagerPluginRegistry> for osquery::ExtensionRegistry {
    fn from(r: ManagerPluginRegistry) -> Self {
        let mut registry = osquery::ExtensionRegistry::new();
        for (plugin_class, plugins) in r.registry {
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

impl ManagerPluginRegistry {
    fn new() -> Self {
        let mut registry = BTreeMap::new();
        use strum::IntoEnumIterator;
        for variant in crate::plugin::PluginVariant::iter() {
            registry.insert(variant.into(), BTreeMap::new());
        }
        Self { registry }
    }
}
