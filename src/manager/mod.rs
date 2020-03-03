mod channel;
mod client;
mod server;

use crate::plugin::{Plugin, PluginVariant};
use std::collections::BTreeMap;

pub type PluginRegistry = BTreeMap<PluginVariant, BTreeMap<String, Box<dyn Plugin>>>;
