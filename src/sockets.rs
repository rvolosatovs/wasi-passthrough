use crate::bindings::exports;
use crate::bindings::wasi::clocks::monotonic_clock::Duration;
use crate::bindings::wasi::sockets::ip_name_lookup::ResolveAddressStream;
use crate::bindings::wasi::sockets::network::{
    ErrorCode, IpAddress, IpAddressFamily, IpSocketAddress, Network,
};
use crate::bindings::wasi::sockets::tcp::TcpSocket;
use crate::bindings::wasi::sockets::udp::{
    IncomingDatagramStream, OutgoingDatagramStream, UdpSocket,
};
use crate::Passthrough;

impl<T: ?Sized> exports::wasi::sockets::ip_name_lookup::Guest for Passthrough<T> {
    type ResolveAddressStream = ResolveAddressStream;

    fn resolve_addresses(
        network: &Network,
        name: String,
    ) -> Result<exports::wasi::sockets::ip_name_lookup::ResolveAddressStream, ErrorCode> {
        todo!()
    }
}

impl exports::wasi::sockets::ip_name_lookup::GuestResolveAddressStream for ResolveAddressStream {
    fn resolve_next_address(&self) -> Result<Option<IpAddress>, ErrorCode> {
        todo!()
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        todo!()
    }
}

impl<T: ?Sized> exports::wasi::sockets::tcp::Guest for Passthrough<T> {
    type TcpSocket = TcpSocket;
}

impl exports::wasi::sockets::tcp::GuestTcpSocket for TcpSocket {
    fn start_bind(
        &self,
        network: &Network,
        local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        todo!()
    }

    fn finish_bind(&self) -> Result<(), ErrorCode> {
        todo!()
    }

    fn start_connect(
        &self,
        network: &Network,
        remote_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        todo!()
    }

    fn finish_connect(
        &self,
    ) -> Result<
        (
            exports::wasi::io::streams::InputStream,
            exports::wasi::io::streams::OutputStream,
        ),
        ErrorCode,
    > {
        todo!()
    }

    fn start_listen(&self) -> Result<(), ErrorCode> {
        todo!()
    }

    fn finish_listen(&self) -> Result<(), ErrorCode> {
        todo!()
    }

    fn accept(
        &self,
    ) -> Result<
        (
            exports::wasi::sockets::tcp::TcpSocket,
            exports::wasi::io::streams::InputStream,
            exports::wasi::io::streams::OutputStream,
        ),
        ErrorCode,
    > {
        todo!()
    }

    fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        todo!()
    }

    fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        todo!()
    }

    fn is_listening(&self) -> bool {
        todo!()
    }

    fn address_family(&self) -> IpAddressFamily {
        todo!()
    }

    fn set_listen_backlog_size(&self, value: u64) -> Result<(), ErrorCode> {
        todo!()
    }

    fn keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
        todo!()
    }

    fn set_keep_alive_enabled(&self, value: bool) -> Result<(), ErrorCode> {
        todo!()
    }

    fn keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
        todo!()
    }

    fn set_keep_alive_idle_time(&self, value: Duration) -> Result<(), ErrorCode> {
        todo!()
    }

    fn keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
        todo!()
    }

    fn set_keep_alive_interval(&self, value: Duration) -> Result<(), ErrorCode> {
        todo!()
    }

    fn keep_alive_count(&self) -> Result<u32, ErrorCode> {
        todo!()
    }

    fn set_keep_alive_count(&self, value: u32) -> Result<(), ErrorCode> {
        todo!()
    }

    fn hop_limit(&self) -> Result<u8, ErrorCode> {
        todo!()
    }

    fn set_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
        todo!()
    }

    fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        todo!()
    }

    fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        todo!()
    }

    fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
        todo!()
    }

    fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        todo!()
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        todo!()
    }

    fn shutdown(
        &self,
        shutdown_type: exports::wasi::sockets::tcp::ShutdownType,
    ) -> Result<(), ErrorCode> {
        todo!()
    }
}

impl<T: ?Sized> exports::wasi::sockets::udp::Guest for Passthrough<T> {
    type UdpSocket = UdpSocket;
    type IncomingDatagramStream = IncomingDatagramStream;
    type OutgoingDatagramStream = OutgoingDatagramStream;
}

impl exports::wasi::sockets::udp::GuestUdpSocket for UdpSocket {
    fn start_bind(
        &self,
        network: &Network,
        local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        todo!()
    }

    fn finish_bind(&self) -> Result<(), ErrorCode> {
        todo!()
    }

    fn stream(
        &self,
        remote_address: Option<IpSocketAddress>,
    ) -> Result<
        (
            exports::wasi::sockets::udp::IncomingDatagramStream,
            exports::wasi::sockets::udp::OutgoingDatagramStream,
        ),
        ErrorCode,
    > {
        todo!()
    }

    fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        todo!()
    }

    fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        todo!()
    }

    fn address_family(&self) -> IpAddressFamily {
        todo!()
    }

    fn unicast_hop_limit(&self) -> Result<u8, ErrorCode> {
        todo!()
    }

    fn set_unicast_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
        todo!()
    }

    fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        todo!()
    }

    fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        todo!()
    }

    fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
        todo!()
    }

    fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        todo!()
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        todo!()
    }
}

impl exports::wasi::sockets::udp::GuestIncomingDatagramStream for IncomingDatagramStream {
    fn receive(
        &self,
        max_results: u64,
    ) -> Result<Vec<exports::wasi::sockets::udp::IncomingDatagram>, ErrorCode> {
        todo!()
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        todo!()
    }
}

impl exports::wasi::sockets::udp::GuestOutgoingDatagramStream for OutgoingDatagramStream {
    fn check_send(&self) -> Result<u64, ErrorCode> {
        todo!()
    }

    fn send(
        &self,
        datagrams: Vec<exports::wasi::sockets::udp::OutgoingDatagram>,
    ) -> Result<u64, ErrorCode> {
        todo!()
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        todo!()
    }
}
