use sflow::header_record::ipv4_packet::Ipv4Packet;
use sflow::IPAddress;

#[derive(Serialize, Deserialize, Debug)]
pub struct L3Json {
    pub ip_protocol: u8,
    pub source_addr: String,
    pub dst_addr: String,
}


impl L3Json {
    pub fn l3_from_packet(packet: &Ipv4Packet) -> Self {
        L3Json {
            ip_protocol: packet.protocol,
            source_addr: ip_address_to_string(packet.source_addr),
            dst_addr: ip_address_to_string(packet.dst_addr),
        }
    }
}

fn ip_address_to_string(ip_address: IPAddress) -> String {
    match ip_address {
        IPAddress::IPv4(ipv4) => ipv4.to_string(),
        IPAddress::IPv6(ipv6) => ipv6.to_string()
    }
}