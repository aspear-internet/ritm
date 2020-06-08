use crate::protocol::*;

use std::convert::TryInto;

pub fn get_l3_protocol(buf: &[u8]) -> L3ProtocolType {
    let v = rtrim_bits(buf[0], 4) >> 4;
    if v == 4 {
        return L3ProtocolType::IPv4;
    }
    if v == 6 {
        return L3ProtocolType::IPv6;
    }

    return L3ProtocolType::Unknown;
}

pub fn set_l3_protocol(buf: &mut [u8], proto: L3ProtocolType)
{
    let mut v = ltrim_bits(buf[0], 4);
    match proto {
        L3ProtocolType::IPv4 => v |= 4 << 4,
        L3ProtocolType::IPv6 => v |= 6 << 4,
        L3ProtocolType::Unknown => v |= 0xFF << 4
    }

    buf[0] = v;
}

pub struct IPv4Adapter<'a> { buf: &'a mut [u8] }
pub struct IPv6Adapter<'a> { buf: &'a mut [u8] }

#[allow(unused)]
impl<'a> IPv4Adapter<'_>
{
    pub fn bind(buf: &'a mut [u8]) -> IPv4Adapter {
        debug_assert!(buf.len() >= 20, "bad ipv4 heaader size!");
        return IPv4Adapter { buf };
    }

    pub fn get_src_addr(&self) -> [u8; 0x04] {
        self.buf[0x0c..0x10].try_into().unwrap()
    }

    pub fn get_dst_addr(&self) -> [u8; 0x04] {
        self.buf[0x10..0x14].try_into().unwrap()
    }
    pub fn get_hlen(&self) -> u8 {
        ltrim_bits(self.buf[0], 4) * 4
    }

    pub fn get_tlen(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x02], self.buf[0x03]])
    }

    pub fn get_type_of_service(&self) -> u8 {
        self.buf[0x01]
    }

    pub fn get_frag_ident(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x04], self.buf[0x05]])
    }

    pub fn get_frag_flags(&self) -> u8 {
        rtrim_bits(self.buf[0x06], 5)
    }

    pub fn get_frag_offset(&self) -> u16 {
        ltrim_bits(u16::from_be_bytes([self.buf[0x06], self.buf[0x07]]), 3)
    }

    pub fn get_ttl(&self) -> u8 {
        self.buf[0x08]
    }

    pub fn get_checksum(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x0a], self.buf[0x0b]])
    }

    pub fn get_protocol(&self) -> L4ProtocolType {
        let proto = self.buf[0x09];
        if (proto == 0x06)
        {
            return L4ProtocolType::TCP;
        }

        if (proto == 0x11)
        {
            return L4ProtocolType::UDP;
        }

        if (proto == 0x01)
        {
            return L4ProtocolType::ICMP;
        }

        return L4ProtocolType::Unknown;
    }

    pub fn set_src_addr(&mut self, addr: [u8; 0x04]) {
        self.buf[0x0c..0x10].copy_from_slice(&addr);
    }

    pub fn set_dst_addr(&mut self, addr: [u8; 0x04]) {
        self.buf[0x10..0x14].copy_from_slice(&addr);
    }

    pub fn set_hlen(&mut self, hlen: u8) {
        debug_assert!(hlen % 4 == 0         , "hlen is not aligned to 4!");
        debug_assert!(hlen / 4 <= 0b11110000, "hlen is too large!");
        self.buf[0x00] = rtrim_bits(self.buf[0x00], 4) | hlen >> 4;
    }

    pub fn set_tlen(&mut self, tlen: u16) {
        self.buf[0x02..0x03].copy_from_slice(&tlen.to_be_bytes());
    }

    pub fn set_type_of_service(&mut self, tos: u8) {
        self.buf[0x01] = tos;
    }

    pub fn set_frag_ident(&mut self, data: u16) {
        self.buf[0x04..0x05].copy_from_slice(&data.to_be_bytes());
    }

    pub fn set_frag_flags(&mut self, data: u8) {
        debug_assert!(data <= 7, "fragmentation flag too large!");
        self.buf[0x06] = rtrim_bits(self.buf[0x06], 3) | data;
    }

    pub fn set_frag_offset(&mut self, data: u16) {
        let new_data = rtrim_bits(u16::from_be_bytes([self.buf[0x06], self.buf[0x07]]), 13) | data;
        self.buf[0x06..0x07].copy_from_slice(&new_data.to_be_bytes());
    }

    pub fn set_ttl(&mut self, ttl: u8) {
        self.buf[0x08] = ttl;
    }

    pub fn set_checksum(&mut self, data: u16) {
        self.buf[0x0a..0x0b].copy_from_slice(&data.to_be_bytes());
    }

    pub fn set_protocol(&mut self, proto: L4ProtocolType) {
        let mut cur_proto = self.buf[0x07];
        match proto {
            L4ProtocolType::Unknown => cur_proto = 0xFF,
            L4ProtocolType::TCP     => cur_proto = 0x06,
            L4ProtocolType::UDP     => cur_proto = 0x11,
            L4ProtocolType::ICMP    => cur_proto = 0x01,
        }
    }
}

#[allow(unused)]
impl<'a> IPv6Adapter<'_>
{
    pub fn bind(buf: &'a mut [u8]) -> IPv6Adapter {
        debug_assert!(buf.len() >= 40, "bad ipv6 heaader size!");
        return IPv6Adapter { buf };
    }

    pub fn get_type_of_service(&self) -> u8 {
        self.buf[0x00] << 4 | self.buf[0x01] >> 4
    }

    pub fn get_flow_label(&self) -> u32 {
        ltrim_bits(u32::from_be_bytes([self.buf[0x00], self.buf[0x01], self.buf[0x02], self.buf[0x03]]), 4)
    }

    pub fn get_hlen(&self) -> u8 {
        40 /* ipv6 fixed header length */
    }

    pub fn get_tlen(&self) -> u16 {
        u16::from_be_bytes([self.buf[0x04], self.buf[0x05]]) + self.get_hlen() as u16
    }

    pub fn get_ttl(&self) -> u8 {
        self.buf[0x07]
    }

    pub fn get_protocol(&self) -> L4ProtocolType {
        return L4ProtocolType::TCP;
    }

    pub fn get_src_addr(&self) -> [u8; 0x10] {
        self.buf[0x08..0x18].try_into().unwrap()
    }

    pub fn get_dst_addr(&self) -> [u8; 0x10] {
        self.buf[0x18..0x28].try_into().unwrap()
    }

    pub fn set_type_of_service(&mut self, tos: u8) {
        self.buf[0x00] = rtrim_bits(self.buf[0x00], 4) | tos >> 4;
        self.buf[0x01] = ltrim_bits(self.buf[0x01], 4) | tos << 4;
    }

    pub fn set_flow_label(&mut self, label: u32) {
    }

    pub fn set_tlen(&mut self, tlen: u16) {

    }

    pub fn set_ttl(&mut self, ttl: u8) {

    }

    pub fn set_protocol(&mut self, proto: L4ProtocolType) {

    }
}


#[cfg(test)]
mod tests {
    use crate::protocol::l3protocol::*;

    #[test]
    fn l3_proto_test1() {
        let buffer = include_bytes!("../../asset/l3_packets/ipv4");
        assert_eq!(get_l3_protocol(buffer), L3ProtocolType::IPv4);
    }

    #[test]
    fn l3_proto_test2() {
        let buffer = include_bytes!("../../asset/l3_packets/ipv6");
        assert_eq!(get_l3_protocol(buffer), L3ProtocolType::IPv6);
    }

    #[test]
    fn l3_proto_test3() {
        let buffer = include_bytes!("../../asset/l3_packets/arp");
        assert_eq!(get_l3_protocol(buffer), L3ProtocolType::Unknown);
    }

    #[test]
    fn l3_proto_test4() {
        let mut buffer: [u8; 1] = [0; 1];

        set_l3_protocol(&mut buffer, L3ProtocolType::IPv4);
        assert_eq!(get_l3_protocol(&buffer), L3ProtocolType::IPv4);

        set_l3_protocol(&mut buffer, L3ProtocolType::IPv6);
        assert_eq!(get_l3_protocol(&buffer), L3ProtocolType::IPv6);

        set_l3_protocol(&mut buffer, L3ProtocolType::Unknown);
        assert_eq!(get_l3_protocol(&buffer), L3ProtocolType::Unknown);
    }

    #[test]
    fn l3_ipv4_test1() {
        let mut buffer: [u8; 20] = *b"\x45\x00\x00\x34\x00\x00\x40\x00\x40\x06\x58\x81\xc0\xa8\x02\x6b\xc6\xc7\x58\x68";

        let adapter = IPv4Adapter::bind(&mut buffer);
        assert_eq!(adapter.get_src_addr(), [192, 168, 2 , 107]);
        assert_eq!(adapter.get_dst_addr(), [198, 199, 88, 104]);
        assert_eq!(adapter.get_hlen(), 20);
        assert_eq!(adapter.get_tlen(), 52);
    }

    #[test]
    fn l3_ipv6_test1() {
        let mut buffer: [u8; 40] = *b"\x60\x01\xB2\xFF\x00\x20\x06\x40\xFE\x80\x00\x00\x00\x00\x00\x00\x10\x30\xE1\xF0\
                                      \xDA\xA6\xF2\x46\xFE\x80\x00\x00\x00\x00\x00\x00\x08\x4A\x5A\xEC\xBF\x5A\x16\x0B";

        let adapter = IPv6Adapter::bind(&mut buffer);
        assert_eq!(adapter.get_src_addr(), [0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x30, 0xe1, 0xf0, 0xda, 0xa6, 0xf2, 0x46]);
        assert_eq!(adapter.get_dst_addr(), [0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x4a, 0x5a, 0xec, 0xbf, 0x5a, 0x16, 0x0b]);
        assert_eq!(adapter.get_hlen(), 40);
        assert_eq!(adapter.get_tlen(), 72);
        assert_eq!(adapter.get_type_of_service(), 0x00);
        assert_eq!(adapter.get_flow_label(), 0x1b2ff);
        assert_eq!(adapter.get_ttl(), 64);
    }
}