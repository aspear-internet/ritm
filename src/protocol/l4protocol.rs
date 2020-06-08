use crate::protocol::*;
use crate::protocol::l3protocol::*;

use std::convert::TryInto;

pub fn get_l4_protocol(buf: &mut [u8]) -> L4ProtocolType {
    match get_l3_protocol(buf) {
        L3ProtocolType::IPv4    => return IPv4Adapter::bind(buf).get_protocol(),
        L3ProtocolType::IPv6    => return IPv6Adapter::bind(buf).get_protocol(),
        L3ProtocolType::Unknown => return L4ProtocolType::Unknown
    }
}

pub fn set_l4_protocol(buf: &mut [u8], proto: L4ProtocolType) {
    match get_l3_protocol(buf) {
        L3ProtocolType::IPv4    => IPv4Adapter::bind(buf).set_protocol(proto),
        L3ProtocolType::IPv6    => IPv6Adapter::bind(buf).set_protocol(proto),
        L3ProtocolType::Unknown => panic!("bad l3 protocol type!")
    }
}

pub struct TCPAdapter<'a> { buf: &'a mut [u8] }
pub struct UDPAdapter<'a> { buf: &'a mut [u8] }
pub struct ICMPAdapter<'a> { buf: &'a mut [u8] }
pub struct IGMPAdapter<'a> { buf: &'a mut [u8] }

impl<'a> TCPAdapter<'_> {

}

impl<'a> UDPAdapter<'_> {

}

impl<'a> ICMPAdapter<'_> {

}

impl<'a> IGMPAdapter<'_> {

}