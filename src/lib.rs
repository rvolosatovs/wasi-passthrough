mod bindings;
mod cli;
mod clocks;
mod filesystem;
mod http;
mod io;
mod sockets;

pub struct Passthrough<T: ?Sized>(pub T);
