use std::sync::Arc;
use thrift::server::TProcessor;
use tokio::prelude::*;
use parity_tokio_ipc::Endpoint;

use thrift_async;e

pub struct AsyncManagerServer<P>
where
    P: TProcessor + Send + Sync + 'static,
{
    processor: Arc<P>,
}

struct AsyncProcessor<R, W, P>
where
    R: AsyncRead,
    W: AsyncWrite,
    P: TProcessor,
{
    reader: R,
    writer: W,
    processor: Arc<P>,
}

impl<R, W, P> AsyncProcessor<R, W, P>
where
    R: AsyncRead,
    W: AsyncWrite,
    P: TProcessor,
{
}

type Error = ();
type Result<T> = std::result::Result<T, Error>;

impl<P> AsyncManagerServer<P>
where
    P: TProcessor + Send + Sync + 'static,
{
    pub fn listen(&mut self, address: &str) -> Result<()> {
        unimplemented!()
    }
}
