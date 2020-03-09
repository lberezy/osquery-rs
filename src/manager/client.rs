// Client that connects to osquery

use crate::gen::osquery;
use crate::manager::comms::Channel;
use crate::manager::PluginRegistry;
use crate::plugin::PluginVariant;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

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

/// use parity_tokio_ipc::{Endpoint, dummy_endpoint};
/// use futures::{future, Future, Stream, StreamExt};
/// use tokio::runtime::Runtime;
///
/// fn main() {
///		let mut runtime = Runtime::new().unwrap();
///     let mut endpoint = Endpoint::new(dummy_endpoint());
///     let server = endpoint.incoming()
///         .expect("failed to open up a new pipe/socket")
///         .for_each(|_stream| {
///             println!("Connection received");
///             futures::future::ready(())
///         });
///		runtime.block_on(server)

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
    registry: Arc<Mutex<PluginRegistry>>,
}

impl ExtensionManagerHandler {
    pub fn new(registry: PluginRegistry) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
        }
    }
}

impl osquery::ExtensionSyncHandler for ExtensionManagerHandler {
    fn handle_ping(&self) -> thrift::Result<osquery::ExtensionStatus> {
        Ok(osquery::ExtensionStatus::new(0, String::from("OK"), None))
    }
    fn handle_call(
        &self,
        registry_name: String,
        item: String,
        request: osquery::ExtensionPluginRequest,
    ) -> thrift::Result<osquery::ExtensionResponse> {
        // extract the relevant sub-registry based on "registry" string
        // extract the right plugin from the sub-registry based on "item" string
        // then call the plugin with the request
        // Ok(plugin.call(request))
        let variant = PluginVariant::from_str(&registry_name).unwrap();
        let registry = self.registry.lock().unwrap();

        match registry.get(&variant) {
            Some(subreg) => {
                match subreg.get(&item) {
                    Some(plugin) => plugin.call(request).map_err(|e| {
                        let err_detail = thrift::ApplicationError::new(
                            thrift::ApplicationErrorKind::Unknown,
                            format!("Plugin call error: {:?}", e),
                        );
                        thrift::Error::Application(err_detail)
                    }),
                    None => {
                        //error - Plugin not found
                        let err_detail = thrift::ApplicationError::new(
                            thrift::ApplicationErrorKind::Unknown,
                            format!("Unknown plugin: {}", item),
                        );
                        // user error type is too tricky to use, application isn't quite the right fit tho
                        return Err(thrift::Error::Application(err_detail));
                    }
                }
            }
            None => {
                //error - Registry not found
                let err_detail = thrift::ApplicationError::new(
                    thrift::ApplicationErrorKind::Unknown,
                    format!("Unknown registry: {}", registry_name),
                );
                return Err(thrift::Error::Application(err_detail));
            }
        }
    }

    fn handle_shutdown(&self) -> thrift::Result<()> {
        // TODO: some more stuff here if required?
        Ok(())
    }
}
