use std::net::{ Ipv4Addr, Ipv6Addr,  SocketAddr };
use std::io::{Write, Read};


unsafe impl Send for StackEmulator { }
unsafe impl Sync for StackEmulator { }

struct StackEmulator {
    context: Box<dyn StackEmuContext>
}

trait StackEmuContext {
    fn tx_process(&mut self, packet: Vec<u8>);
    fn tx_out(&mut self) -> Option<Vec<u8>>;

    fn rx_process(&mut self, packet: Vec<u8>);
    fn rx_out(&mut self) -> Option<Vec<u8>>;
}

struct IPv4StackEmuContext {
    src_addr: Ipv4Addr,
    dst_addr: Ipv6Addr,
}

struct IPv6StackEmuContext {

}

struct TCPStackEmuContext {
    l3_layer_ctx: Box<dyn StackEmuContext>,
    src_port: u16,
    dst_port: u16,

    rx_buffer: Vec<Vec<u8>>,
    tx_buffer: Vec<Vec<u8>>,

    // current window size state.
    rx_window_size: u16,
    rx_window_scale: u8,
    tx_window_size: u16,
    tx_window_scale: u16,
}

struct UDPStackEmuContext {
    src_addr: SocketAddr,
    dst_addr: SocketAddr,

    rx_buffer: Vec<Vec<u8>>,
    tx_buffer: Vec<Vec<u8>>,
}

// raw to raw tcp stack emulator
// this manages gaps between modified packet sizes, like seq, ack number, window size.
struct TCPRawStackEmuContext {

}

// raw to raw udp stack emulator.
struct UDPRawStackEmuContext {

}