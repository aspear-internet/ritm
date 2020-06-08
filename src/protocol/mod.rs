pub mod l3protocol;
pub mod l4protocol;
pub mod l7protocol;

use num_traits::int::PrimInt;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum L3ProtocolType
{
    Unknown, IPv4, IPv6
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum L4ProtocolType
{
    Unknown, TCP, UDP, ICMP
}

impl L4ProtocolType {
    pub fn from_proto(proto: u8) -> L4ProtocolType {
        match proto {
            0x06 => return L4ProtocolType::TCP,
            0x11 => return L4ProtocolType::UDP,
            0x01 => return L4ProtocolType::ICMP,
            _    => return L4ProtocolType::Unknown
        };
    }

    pub fn to_proto(proto: L4ProtocolType) -> u8 {
        match proto {
            L4ProtocolType::Unknown => return 0xFFu8,
            L4ProtocolType::TCP     => return 0x06u8,
            L4ProtocolType::UDP     => return 0x11u8,
            L4ProtocolType::ICMP    => return 0x01u8,
        };
    }
}

#[inline]
pub fn rtrim_bits<T: PrimInt>(data: T, bits: usize) -> T
{
    return (data >> bits) << bits;
}

#[inline]
pub fn ltrim_bits<T: PrimInt>(data: T, bits: usize) -> T
{
    return (data << bits) >> bits;
}
