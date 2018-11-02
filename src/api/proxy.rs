use std::net::*;
use std::io::{Write, Error, ErrorKind};
use atomicwrites::{AtomicFile,AllowOverwrite};
use domain::bits::name::FromStrError;


use api::*;
use zone::*;


/// Web proxy wrapper:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proxy {

    /// Proxy belocngs to cell with name:
    pub cell: Option<String>,

    /// Proxy from URL:
    pub from: Option<String>,

    /// Resolved IPv4 from "from" URL:
    pub from_ipv4: Option<IpAddr>,

    /// Proxy to URL:
    pub to: Option<String>,

    /// Resolved IPv4 from "to" URL:
    pub to_ipv4: Option<IpAddr>,

    /// Generated Nginx-Proxy configuration:
    pub config: Option<String>,

}


impl Default for Proxy {
    fn default() -> Proxy {
        Proxy{
            cell: None,
            config: None,
            from: None,
            from_ipv4: None,
            to: None,
            to_ipv4: None,
        }
    }
}


/// Serialize to JSON on .to_string()
impl ToString for Proxy {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


impl Proxy {


    /// Empty proxy
    pub fn empty() -> Result<Proxy, Error> {
        Ok(Proxy::default())
    }


    /// Create new Web Proxy configuration:
    pub fn new(cell_name: &String, from: &String, to: &String) -> Result<Proxy, Error> {
        Zone::validate_domain_addresses(from, to)
            .and_then(|(valid_ipv4_from, valid_ipv4_to)| {
                // When both domains are valid, create Proxy object:
                let proxy = Proxy {
                    cell: Some(cell_name.to_string()),
                    config: Some(Proxy::config_from_template(from, to)),
                    from: Some(from.to_string()),
                    from_ipv4: Some(valid_ipv4_from),
                    to: Some(to.to_string()),
                    to_ipv4: Some(valid_ipv4_to),
                };
                Ok(proxy)
            })
            .map_err(|err| {
                error!("{}", err);
                Error::new(ErrorKind::Other, err)
            })
    }


    /// Nginx proxy config from predefined template:
    pub fn config_from_template(from_domain: &String, to_domain: &String) -> String {
        /*
         * external/public domain is proxied to internal one
         * We explicitly set Nginx to re-resolve domains
         * using our local DNS server
         */
        format!(r"
server {{
    listen 80;
    server_name {};

    resolver {} valid=15s;
    set $backend 'http://{}';

    location / {{
        proxy_pass $backend;
    }}

    access_log off;
}}
    ",
        from_domain,
        &DEFAULT_DNS.parse().unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))), // resolver
        to_domain)
    }


}


