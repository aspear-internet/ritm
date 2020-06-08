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
