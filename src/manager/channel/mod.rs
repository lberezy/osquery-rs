#[cfg(target_os = "windows")]
mod namedpipe;
// #[cfg(target_os = "unix")]
mod socket;

use thrift::protocol::{TInputProtocol, TOutputProtocol};

pub trait Channel {
    type Output: TOutputProtocol;
    type Input: TInputProtocol;

    fn split(self) -> (Self::Input, Self::Output);
}
