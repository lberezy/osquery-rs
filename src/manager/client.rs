// Client that connects to osquery

use crate::gen::osquery;
use crate::manager::channel::Channel;
use crate::manager::PluginRegistry;
use crate::plugin::PluginVariant;
pub struct ExtensionManagerClient<C: Channel> {
    client: osquery::ExtensionManagerSyncClient<C::Input, C::Output>,
}

impl<C> ExtensionManagerClient<C>
where
    C: Channel,
{
    pub fn new_with_channel(channel: C) -> Self {
        let (input, output) = channel.split();
        let client = osquery::ExtensionManagerSyncClient::new(input, output);
        ExtensionManagerClient { client }
    }
}

impl<C> osquery::TExtensionManagerSyncClient for ExtensionManagerClient<C>
where
    C: Channel,
{
    fn extensions(&mut self) -> thrift::Result<osquery::InternalExtensionList> {
        self.client.extensions()
    }
    fn options(&mut self) -> thrift::Result<osquery::InternalOptionList> {
        self.client.options()
    }
    fn register_extension(
        &mut self,
        info: osquery::InternalExtensionInfo,
        registry: osquery::ExtensionRegistry,
    ) -> thrift::Result<osquery::ExtensionStatus> {
        self.client.register_extension(info, registry)
    }
    fn deregister_extension(
        &mut self,
        uuid: osquery::ExtensionRouteUUID,
    ) -> thrift::Result<osquery::ExtensionStatus> {
        self.client.deregister_extension(uuid)
    }
    fn query(&mut self, sql: String) -> thrift::Result<osquery::ExtensionResponse> {
        self.client.query(sql)
    }
    fn get_query_columns(&mut self, sql: String) -> thrift::Result<osquery::ExtensionResponse> {
        self.client.get_query_columns(sql)
    }
}

impl<C> osquery::TExtensionSyncClient for ExtensionManagerClient<C>
where
    C: Channel,
{
    fn ping(&mut self) -> thrift::Result<osquery::ExtensionStatus> {
        self.client.ping()
    }
    fn call(
        &mut self,
        registry: String,
        item: String,
        request: osquery::ExtensionPluginRequest,
    ) -> thrift::Result<osquery::ExtensionResponse> {
        self.client.call(registry, item, request)
    }
    fn shutdown(&mut self) -> thrift::Result<()> {
        self.client.shutdown()
    }
}

impl<C> Drop for ExtensionManagerClient<C>
where
    C: Channel,
{
    fn drop(&mut self) {
        dbg!("Dropping client.");
        // TODO: Maybe cleanup channel?
        //self.channel._
    }
}

///////////////////////// Handler ////////////////////////////////////////////

pub struct ExtensionManagerHandler {
    // TODO: likely need to make this send + sync, i.e. Arc<Mutex<_>>
    registry: PluginRegistry,
}

impl ExtensionManagerHandler {
    pub fn new(registry: PluginRegistry) -> Self {
        Self { registry }
    }
}

impl osquery::ExtensionSyncHandler for ExtensionManagerHandler {
    fn handle_ping(&self) -> thrift::Result<osquery::ExtensionStatus> {
        Ok(osquery::ExtensionStatus::new(0, String::from("OK"), None))
    }
    fn handle_call(
        &self,
        registry: String,
        _item: String,
        _request: osquery::ExtensionPluginRequest,
    ) -> thrift::Result<osquery::ExtensionResponse> {
        //self.registry.contains_key()
        //let variant = PluginVariant::from_str(registry).unwrap();

        // extract the relevant sub-registry based on "registry" string
        // extract the right plugin from the sub-registry based on "item" string
        // then call the plugin with the request
        // Ok(plugin.call(request))
        unimplemented!()
    }
    fn handle_shutdown(&self) -> thrift::Result<()> {
        // TODO: some more stuff here if required?
        Ok(())
    }
}
