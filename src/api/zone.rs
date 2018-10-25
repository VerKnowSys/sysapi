#[derive(Debug, Serialize)]
enum ZoneTypes {
    A_,
    Cname,
    Txt,
}


#[derive(Debug, Serialize)]
pub struct Zone {
    domain_name: String,
    ipv4: String,
    zone_type: ZoneTypes,
}
