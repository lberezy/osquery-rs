mod table;
use crate::gen::osquery;
use crate::gen::osquery::{ExtensionPluginRequest, ExtensionPluginResponse, ExtensionStatus};
use strum_macros::EnumIter;

// TODO: add more
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum PluginVariant {
    Table,
}

impl From<PluginVariant> for String {
    fn from(p: PluginVariant) -> Self {
        match p {
            PluginVariant::Table => String::from("table"),
        }
    }
}

pub trait Plugin {
    fn call(
        &self,
        request: ExtensionPluginRequest,
    ) -> std::result::Result<osquery::ExtensionResponse, crate::error::Error>;

    fn routes(&self) -> ExtensionPluginResponse;

    fn name(&self) -> &str;

    fn registry_name(&self) -> &'static str;

    fn ping(&self) -> ExtensionStatus {
        ExtensionStatus::new(
            0,
            String::from("OK"),
            osquery::ExtensionRouteUUID::default(),
        )
    }

    fn shutdown(&self) {}
}
