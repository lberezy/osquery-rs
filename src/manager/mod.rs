mod channel;
mod client;
mod server;

use crate::plugin::Plugin;
use std::collections::BTreeMap;

pub type PluginRegistry = BTreeMap<String, BTreeMap<String, Box<dyn Plugin>>>;
