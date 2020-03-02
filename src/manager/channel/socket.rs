use super::Channel;

use std::os::unix::net::UnixStream;
use std::path::Path;
use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};

use thrift::transport::{ReadHalf, TFramedReadTransport, TFramedWriteTransport, WriteHalf};

use crate::error::ManagerError;

pub struct SocketChannel {
    channel: (ReadHalf<UnixStream>, WriteHalf<UnixStream>),
}

impl SocketChannel {
    pub fn connect<P: AsRef<Path>>(path: P) -> Result<Self, ManagerError> {
        UnixStream::connect(path)
            .and_then(|stream| stream.try_clone().map(|clone| (clone, stream)))
            .map_err(ManagerError::from)
            .and_then(|(cloned, stream)| {
                let read_half = ReadHalf::new(stream);
                let write_half = WriteHalf::new(cloned);
                Ok(Ok(SocketChannel {
                    channel: (read_half, write_half),
                }))
            })
            .map_err(|_| ManagerError::GenericIo)?
    }
}

impl Channel for SocketChannel {
    type Input = TBinaryInputProtocol<TFramedReadTransport<ReadHalf<UnixStream>>>;
    type Output = TBinaryOutputProtocol<TFramedWriteTransport<WriteHalf<UnixStream>>>;

    fn split(self) -> (Self::Input, Self::Output) {
        let (read_chan, write_chan) = self.channel;
        let i_tran = TFramedReadTransport::new(read_chan);
        let o_tran = TFramedWriteTransport::new(write_chan);
        let i_prot = TBinaryInputProtocol::new(i_tran, true);
        let o_prot = TBinaryOutputProtocol::new(o_tran, true);
        (i_prot, o_prot)
    }
}
