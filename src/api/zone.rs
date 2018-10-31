#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
enum ZoneTypes {
    A_,
    Cname,
    Txt,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Zone {
    domain_name: Option<String>,
    ipv4: Option<String>,
    zone_type: Option<ZoneTypes>,
}
