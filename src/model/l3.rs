use sflow::header_record::ipv4_packet::Ipv4Packet;

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
            source_addr: "1.1.1.1".to_string(),//todo:bcm - fix
            dst_addr: "1.1.1.1".to_string(),//todo:bcm - fix
        }
    }
}