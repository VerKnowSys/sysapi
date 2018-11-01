use std::net::*;
use std::io::{Error, ErrorKind};


use api::*;
use zone::*;


/// Web proxy wrapper:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proxy {

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
    pub fn new(from: &String, to: &String) -> Result<Proxy, Error> {
        // Validate both domains, and if both Zones are valid, create Proxy object:
        match Zone::validate_domain_addresses(from, to) {
            Ok((valid_ipv4_from, valid_ipv4_to)) => {
                let new_proxy_conf = Proxy::config_from_template(from, to);
                debug!("New Web-Proxy configuration block:\n{}\n", new_proxy_conf);
                let proxy = Proxy {
                    config: Some(new_proxy_conf),
                    from: Some(from.to_string()),
                    from_ipv4: Some(valid_ipv4_from),
                    to: Some(to.to_string()),
                    to_ipv4: Some(valid_ipv4_to),
                };
                debug!("Proxy object: {}", proxy.to_string());
                Ok(proxy)
            },
            Err(err) => {
                error!("{}", err);
                Err(Error::new(ErrorKind::Other, err))
            }
        }
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


