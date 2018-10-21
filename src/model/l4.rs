use sflow::header_record::layer4::l4::Layer4Packet;

#[derive(Serialize, Deserialize, Debug)]
pub struct L4Json {
    pub l4_type: L4JsonType,
    pub icmp_type: Option<u8>,
    pub icmp_code: Option<u8>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub ack: Option<bool>,
    pub syn: Option<bool>,
    pub rst: Option<bool>,
    pub fin: Option<bool>,
    pub window_size: Option<u16>,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum L4JsonType {
    Icmp,
    Tcp,
    Udp,
    Unknown,
}

impl L4Json {
    pub fn l4_from_packet(packet: &Layer4Packet) -> Self {
        match packet {
            Layer4Packet::Icmp(icmp) =>
                L4Json {
                    l4_type: L4JsonType::Icmp,
                    icmp_type: Some(icmp.icmp_type),
                    icmp_code: Some(icmp.icmp_code),
                    src_port: None,
                    dst_port: None,
                    ack: None,
                    syn: None,
                    rst: None,
                    fin: None,
                    window_size: None,
                },
            Layer4Packet::TCP(tcp) => L4Json {
                l4_type: L4JsonType::Tcp,
                icmp_type: None,
                icmp_code: None,
                src_port: Some(tcp.src_port),
                dst_port: Some(tcp.dst_port),
                ack: Some(tcp.ack),
                syn: Some(tcp.syn),
                rst: Some(tcp.rst),
                fin: Some(tcp.fin),
                window_size: Some(tcp.window_size),
            },
            Layer4Packet::UDP() => L4Json {
                l4_type: L4JsonType::Udp, //todo - add udp
                icmp_type: None,
                icmp_code: None,
                src_port: None,
                dst_port: None,
                ack: None,
                syn: None,
                rst: None,
                fin: None,
                window_size: None,
            },
            Layer4Packet::Unknown => L4Json {
                l4_type: L4JsonType::Icmp,
                icmp_type: None,
                icmp_code: None,
                src_port: None,
                dst_port: None,
                ack: None,
                syn: None,
                rst: None,
                fin: None,
                window_size: None,
            }
        }
    }
}