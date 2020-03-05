use super::client::ExtensionManagerClient;
use super::comms::Channel;
use super::{ManagerPluginRegistry, Plugin};

use crate::{gen::osquery, manager::client::ExtensionManagerHandler};
use std::sync::Arc;
use thrift::server::TProcessor;

use std::collections::BTreeMap;
use std::path::Path;
use threadpool::ThreadPool;

pub struct ExtensionManagerServer<C: Channel> {
    name: String,
    client: ExtensionManagerClient<C>,
    registry: super::PluginRegistry,
    endpoint_path: Path,
}

// impl<C> ExtensionManagerServer<C>
// where
//     C: Channel,
// {
//     // fn new_from_channel(name: &str, channel: C) -> Self {
//     //     Self {
//     //         name: name.into(),
//     //         client: client,
//     //         registry: ManagerPluginRegistry::new(),
//     //     }
//     // }

//     // pub fn new<P: AsRef<Path>>(name: &str, endpoint: P) -> Self {
//     //     // let channel =  ;

//     //     let client = ExtensionManagerClient::new_with_channel(channel);

//     //     Self {
//     //         name: name.into(),
//     //         client: client,
//     //         registry: ManagerPluginRegistry::new(),
//     //         endpoint_path: endpoint,
//     //     }
//     // }

//     /// Register plugins with the server
//     pub fn register(&mut self, plugin: Box<dyn Plugin>) {
//         let plugin_reg_name = plugin.registry_name();
//         if self.registry.contains_key(&plugin_reg_name) {
//             // get the appropriate sub-registry and insert the plugin by name
//             self.registry
//                 .get_mut(&plugin_reg_name)
//                 .and_then(|s| Some(s.insert(plugin.name().into(), plugin)))
//                 .map(|_| unreachable!())
//                 .unwrap();
//         }
//         panic!("Unsupported plugin type: {:?}", plugin_reg_name);
//     }

//     /// Call this to start related plugins
//     pub fn run(self) {
//         // TODO: All of this

//         // build registry
//         let registry = self.build_registry();

//         // create client
//         // register plugin registry with via client
//         // setup handler and router/processor

//         let (input, output) =
//             crate::manager::comms::socket::SocketChannel::connect(self.endpoint_path)
//                 .unwrap()
//                 .split()
//                 .into();

//         let handler = ExtensionManagerHandler::new(registry);
//         let processor = osquery::ExtensionSyncProcessor::new(handler);
//         // create a new channel and feed to processor inside a bunch fo threads/some executor
//         processor.process(&mut input, &mut output);

//         // wait in another thread for pings from osquery, send signal to stop if no heartbeat

//         unimplemented!()
//     }
// }

// TODO: Reimplement thrift::server::TServer but with a generic listener type to handle platform specific listeners (UnixListener on Unix and NamedPipeListener?? on Windows)
// #[derive(Debug)]
// pub struct ProcessorServer<P>
// where
//     P: TProcessor + Send + Sync + 'static,
//     // TODO: Resolve this design decision
//     // L: crate::manager::comms::platform::Listener,
// {
//     processor: Arc<P>,
//     workers: ThreadPool,
// }

// impl<P, L> ProcessorServer<P, L>
// where
//     P: TProcessor + Send + Sync + 'static,
// {
//     pub fn new(processor: P) -> ProcessorServer<P, L> {
//         Self {
//             processor: Arc::new(processor),
//             // TODO: Parameterise number of threads
//             workers: ThreadPool::new(4),
//         }
//     }

//     // pub fn listen(&mut self) -> thrift::Result<()> {
//     //     // create listener
//     //     for stream in listener.incoming() {
//     //         match stream {
//     //             Ok(s) => {
//     //                 let (i_prot, o_prot) = self.new_protocols_for_connection(s)?;
//     //                 let processor = self.processor.clone();
//     //                 self.worker_pool
//     //                     .execute(move || handle_incoming_connection(processor, i_prot, o_prot));
//     //             }
//     //             Err(e) => {
//     //                 warn!("failed to accept remote connection with error {:?}", e);
//     //             }
//     //         }
//     //     }

//     //     Err(thrift::Error::Application(thrift::ApplicationError {
//     //         kind: thrift::ApplicationErrorKind::Unknown,
//     //         message: "aborted listen loop".into(),
//     //     }))
//     // }

//     // fn new_protocols_for_connection(
//     //     &mut self,
//     //     stream: TcpStream,
//     // ) -> ::Result<(Box<dyn TInputProtocol + Send>, Box<dyn TOutputProtocol + Send>)> {
//     //     // create the shared tcp stream
//     //     let channel = TTcpChannel::with_stream(stream);

//     //     // split it into two - one to be owned by the
//     //     // input tran/proto and the other by the output
//     //     let (r_chan, w_chan) = channel.split()?;

//     //     // input protocol and transport
//     //     let r_tran = self.r_trans_factory.create(Box::new(r_chan));
//     //     let i_prot = self.i_proto_factory.create(r_tran);

//     //     // output protocol and transport
//     //     let w_tran = self.w_trans_factory.create(Box::new(w_chan));
//     //     let o_prot = self.o_proto_factory.create(w_tran);

//     //     Ok((i_prot, o_prot))
//     // }
// }

// fn handle_incoming_connection<PRC>(
//     processor: Arc<PRC>,
//     i_prot: Box<dyn TInputProtocol>,
//     o_prot: Box<dyn TOutputProtocol>,
// ) where
//     PRC: TProcessor,
// {
//     let mut i_prot = i_prot;
//     let mut o_prot = o_prot;
//     loop {
//         let r = processor.process(&mut *i_prot, &mut *o_prot);
//         if let Err(e) = r {
//             warn!("processor completed with error: {:?}", e);
//             break;
//         }
//     }
// }
