use model::l3::L3Json;
use model::l4::L4Json;
use model::l7::L7Json;
use sflow::flow_records::SampledHeader;
use sflow::header_record::layer4::l4::Layer4Packet;

#[derive(Serialize, Deserialize, Debug)]
pub struct PacketJson {
    l3: L3Json,
    l4: L4Json,
    l7: Option<L7Json>,
}

impl PacketJson {
    pub fn from_sampled_header(header: &SampledHeader) -> Self {
        let ipv4 = &(*&(*header).packet).packet; //todo:bcm - fiox this shit
        let ipv4json = L3Json::l3_from_packet(&ipv4);

        let l4 = &(*ipv4).content;
        let l4json = L4Json::l4_from_packet(&l4);

        let l7 = match l4 {
            Layer4Packet::TCP(tcp) => Some(&(*tcp).data),
            _ => None
        };
        let l7json = l7.map(|packet| L7Json::from_l7_packet(&packet));

        PacketJson { l3: ipv4json, l4: l4json, l7: l7json }
    }
}