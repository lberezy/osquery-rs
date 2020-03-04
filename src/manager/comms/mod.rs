#[cfg(target_family = "windows")]
pub mod namedpipe;
#[cfg(target_family = "unix")]
pub mod socket;

use thrift::protocol::{TInputProtocol, TOutputProtocol};

pub trait Channel {
    type Output: TOutputProtocol;
    type Input: TInputProtocol;

    fn split(self) -> (Self::Input, Self::Output);
}

// TODO: This is a bad design and will cause problems, resolve this
#[cfg(target_family = "windows")]
pub mod platform {
    // pub type Listener = namedpipe::
}

#[cfg(target_family = "unix")]
pub mod platform {
    pub type Listener = std::os::unix::net::UnixListener;
}
