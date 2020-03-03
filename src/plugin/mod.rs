mod table;
use crate::gen::osquery;
use crate::gen::osquery::{ExtensionPluginRequest, ExtensionPluginResponse, ExtensionStatus};

use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

// TODO: add more
#[derive(
    Ord,
    Eq,
    PartialOrd,
    PartialEq,
    Debug,
    Copy,
    Clone,
    EnumIter,
    IntoStaticStr,
    AsRefStr,
    Display,
    EnumString,
)]
pub enum PluginVariant {
    #[strum(serialize = "table")]
    Table,
}

pub trait Plugin {
    fn call(
        &self,
        request: ExtensionPluginRequest,
    ) -> std::result::Result<osquery::ExtensionResponse, crate::error::Error>;

    fn routes(&self) -> ExtensionPluginResponse;

    /// Returns the name used to refer to the plugin (e.g. table name implemented by the plugin)
    fn name(&self) -> &str;

    /// Returns the variant of the plugin for use in the registry. (e.g. "table", "logger", etc.)
    fn registry_name(&self) -> PluginVariant;

    fn ping(&self) -> ExtensionStatus {
        ExtensionStatus::new(
            0,
            String::from("OK"),
            osquery::ExtensionRouteUUID::default(),
        )
    }

    fn shutdown(&self) {}
}
