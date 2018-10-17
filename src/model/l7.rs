use sflow::header_record::layer7::l7::Layer7Packet;

#[derive(Serialize, Deserialize, Debug)]
pub struct L7Json {
    pub packet_type: L7JsonType,
    pub method: Option<String>,
    pub path: Option<String>,
    pub host: Option<String>,
    pub status_code: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum L7JsonType {
    HttpRequest,
    HttpResponse,
    Unknown,
}

impl L7Json {
    pub fn from_l7_packet(packet: &Layer7Packet) -> Self {
        return match packet {
            Layer7Packet::HttpReq(request) =>
                L7Json {
                    packet_type: L7JsonType::HttpRequest,
                    status_code: None,
                    method: Some(request.method.clone()),
                    path: Some(request.path.clone()),
                    host: request.host.clone(),
                },
            Layer7Packet::HttpResp(response) =>
                L7Json {
                    packet_type: L7JsonType::HttpResponse,
                    status_code: Some(response.status_code),
                    method: None,
                    path: None,
                    host: None,
                },
            Layer7Packet::Unknown => L7Json {
                packet_type: L7JsonType::Unknown,
                status_code: None,
                method: None,
                path: None,
                host: None,
            }
        };
    }
}
