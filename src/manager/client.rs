// Client that connects to osquery
use super::PluginRegistry;
use crate::gen::osquery::ExtensionManagerSyncClient;
use crate::manager::channel::Channel;

pub struct Client<C: Channel> {
    client: ExtensionManagerSyncClient<C::Input, C::Output>,
    // channel: C,
}

impl<C> Client<C>
where
    C: Channel,
{
    pub fn new_with_channel(channel: C) -> Self {
        let (input, output) = channel.split();
        let client = ExtensionManagerSyncClient::new(input, output);
        Client { client }
    }
}

impl<C> Drop for Client<C>
where
    C: Channel,
{
    fn drop(&mut self) {
        dbg!("Dropping client.");
        // TODO: Maybe cleanup channel?
        //self.channel._
    }
}
