use domain::bits::DNameBuf;
use domain::bits::name::*;
use domain::resolv::{Resolver, ResolvConf};
use domain::resolv::conf::ServerConf;
use domain::resolv::lookup::lookup_host;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;


use api::*;


/// DNS Zone types:
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
enum ZoneTypes {
    A_,
    Cname,
    Txt,
}


/// DNS Zone representation:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Zone {
    domain_name: Option<String>,
    ipv4: Option<String>,
    zone_type: Option<ZoneTypes>,
}


impl Zone {


    /// Validate IPv4 address of given domain:
    pub fn lookup_domain(domain: &String) -> Result<IpAddr, FromStrError> {
        let ip_localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let default_dns = &DEFAULT_DNS.parse().unwrap_or(ip_localhost);
        let dns_address = SocketAddr::new(*default_dns, 53);
        let svrconf_default = ServerConf::new(dns_address);
        let resolver_conf = ResolvConf {
            servers: vec!(svrconf_default),
            .. ResolvConf::default()
        };

        DNameBuf::from_str(domain)
            .and_then(|from_domain| {
                Resolver::run_with_conf(resolver_conf, |resolv| lookup_host(resolv, &from_domain))
                    .and_then(|ipv4| {
                        let ipv4_from = ipv4
                            .iter()
                            .next()
                            .unwrap_or(ip_localhost);
                        debug!("Proxy: Domain -> IpV4: {:?} -> {:?}", from_domain, ipv4_from);
                        if from_domain != ipv4.canonical_name() {
                            info!("Proxy::new(): Domain: {} is an alias for: {}",
                                  from_domain, ipv4.canonical_name());

                            Ok(ipv4_from)
                        } else {
                            info!("Proxy::new(): Domain: {} has address: {:?}", ipv4.canonical_name(), ipv4_from);
                            Ok(ipv4_from)
                        }
                    })
                    .map_err(|err| {
                        error!("Empty DNS resolve. Error: {}", err);
                        FromStrError::EmptyLabel
                    })
            })
    }


}
