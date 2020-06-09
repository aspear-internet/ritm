use crate::protocol::*;
use crate::protocol::l3protocol::*;

use bitflags::bitflags;
use std::convert::TryInto;

bitflags! {
    pub struct TCPFlags: u8 {
        const FIN = 0b00000001;
        const SYN = 0b00000010;
        const RST = 0b00000100;
        const PSH = 0b00001000;
        const ACK = 0b00010000;
        const URG = 0b00100000;
        const ECE = 0b01000000;
        const CWR = 0b10000000;
    }
}

pub fn get_l4_protocol(buf: &mut [u8]) -> L4ProtocolType {
    match get_l3_protocol(buf) {
        L3ProtocolType::IPv4    => return IPv4Adapter::bind(buf).get_protocol(),
        L3ProtocolType::IPv6    => return IPv6Adapter::bind(buf).get_protocol(),
        L3ProtocolType::Unknown => unreachable!()
    }
}

pub fn set_l4_protocol(buf: &mut [u8], proto: L4ProtocolType) {
    match get_l3_protocol(buf) {
        L3ProtocolType::IPv4    => IPv4Adapter::bind(buf).set_protocol(proto),
        L3ProtocolType::IPv6    => IPv6Adapter::bind(buf).set_protocol(proto),
        L3ProtocolType::Unknown => unreachable!()
    }
}

pub struct TCPAdapter<'a> { buf: &'a mut [u8] }
pub struct UDPAdapter<'a> { buf: &'a mut [u8] }
pub struct ICMPAdapter<'a> { buf: &'a mut [u8] }
pub struct IGMPAdapter<'a> { buf: &'a mut [u8] }

#[allow(unused)]
impl<'a> TCPAdapter<'_> {
    pub fn bind(buf: &'a mut [u8]) -> TCPAdapter {
        return TCPAdapter { buf };
    }

    pub fn get_src_port(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x00], self.buf[0x01]])
    }

    pub fn get_dst_port(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x02], self.buf[0x03]])
    }

    pub fn get_seq_num(&self) -> u32 {
        u32::from_be_bytes([self.buf[0x04], self.buf[0x05], self.buf[0x06], self.buf[0x07]])
    }

    pub fn get_ack_num(&self) -> u32 {
        u32::from_be_bytes([self.buf[0x08], self.buf[0x09], self.buf[0x0a], self.buf[0x0b]])
    }

    pub fn get_hlen(&self) -> u8 {
        (self.buf[0x0c] >> 4) * 4
    }

    pub fn get_flags(&self) -> TCPFlags {
        TCPFlags::from_bits(self.buf[0x0d]).unwrap()
    }

    pub fn has_flags(&self, flag: TCPFlags) -> bool  {
        (self.get_flags() & flag) == flag
    }

    pub fn get_win_sz(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x0e], self.buf[0x0f]])
    }

    pub fn get_checksum(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x10], self.buf[0x11]])
    }
}

#[allow(unused)]
impl<'a> UDPAdapter<'_> {
    pub fn bind(buf: &'a mut [u8]) -> UDPAdapter {
        return UDPAdapter { buf };
    }

    pub fn get_src_port(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x00], self.buf[0x01]])
    }

    pub fn get_dst_port(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x02], self.buf[0x03]])
    }

    pub fn get_tlen(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x04], self.buf[0x05]])
    }

    pub fn get_checksum(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x06], self.buf[0x07]])
    }
}

#[allow(unused)]
impl<'a> ICMPAdapter<'_> {

}

#[allow(unused)]
impl<'a> IGMPAdapter<'_> {

}

#[cfg(test)]
mod l4_proto_tests {
    use crate::protocol::l4protocol::*;

    #[test]
    fn l4_tcp_test1() {

    }

    #[test]
    fn l4_udp_test1() {
        let mut buffer = *b"\x00\x35\xf0\x3c\x00\x9a\xa3\xa4";

        let adapter = UDPAdapter::bind(&mut buffer);
        assert_eq!(adapter.get_src_port(), 53);
        assert_eq!(adapter.get_dst_port(), 61500);
        assert_eq!(adapter.get_tlen()    , 154);
        assert_eq!(adapter.get_checksum(), 0xa3a4);
    }
}