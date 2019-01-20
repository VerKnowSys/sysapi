use domain::bits::name::FromStrError;
use std::net::{IpAddr, Ipv4Addr};
use abstract_ns::HostResolve;
use ns_std_threaded::ThreadedResolver;
use futures::future::*;
use colored::Colorize;


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


    /// Validate each domain pair (from => to)— has also valid/resolvable/non-local address:
    pub fn validate_domain_addresses(from: &str, to: &str) -> Result<(IpAddr, IpAddr), FromStrError> {
        Zone::lookup_domain(from)
            .and_then(|valid_ipv4_from| {
                Zone::lookup_domain(to)
                    .and_then(|valid_ipv4_to| {
                        if !valid_ipv4_from.is_ipv4()
                        || valid_ipv4_from.is_loopback()
                        || valid_ipv4_from.is_unspecified()
                        || valid_ipv4_from.is_multicast()
                        || !valid_ipv4_to.is_ipv4()
                        || valid_ipv4_to.is_loopback()
                        || valid_ipv4_to.is_unspecified()
                        || valid_ipv4_to.is_multicast() {
                            let validate_state = format!("FROM:   Ipv4: {}, Lpbck: {}, Wldcrd: {}, Mltcst: {}",
                                !valid_ipv4_from.is_ipv4(),
                                valid_ipv4_from.is_loopback(),
                                valid_ipv4_from.is_unspecified(),
                                valid_ipv4_from.is_multicast());
                            let validate_state2 = format!("TO:   Ipv4: {}, Lpbck: {}, Wldcrd: {}, Mltcst: {}",
                                !valid_ipv4_to.is_ipv4(),
                                valid_ipv4_to.is_loopback(),
                                valid_ipv4_to.is_unspecified(),
                                valid_ipv4_to.is_multicast());
                            let err_msg = format!("validate_domain_addresses(): Validation failed for pair: {} -> {}. Validation details:\n\n{}\n{}\n",
                                                  valid_ipv4_from.to_string().yellow(), valid_ipv4_to.to_string().yellow(),
                                                  validate_state.to_string().cyan(), validate_state2.to_string().cyan());
                            error!("{}", err_msg);
                            Err(FromStrError::EmptyLabel)
                        } else {
                            debug!("validate_domain_addresses(): IPv4 pair: {} -> {}",
                                   valid_ipv4_from.to_string().cyan(), valid_ipv4_to.to_string().cyan());
                            Ok((valid_ipv4_from, valid_ipv4_to))
                        }
                    })
            })
    }


    /// Asynchronously resolves first available IPv4 defined for given "domain":
    pub fn lookup_domain(domain: &str) -> Result<IpAddr, FromStrError> {
        let ip_localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)); // Used as fallback if domain resolve fails
        let response = ThreadedResolver::new()
            .resolve_host(&domain.parse().unwrap())
            .map(|addresses| {
                debug!("Domain: {} resolves to IPv4(s): [{}]", &domain.cyan(), &addresses.iter().map(|e| e.to_string()).collect::<String>().cyan());
                addresses
                    .iter()
                    .find(|&element| !element.to_string().contains(':')) /* NOTE: filter out IPv6 - unsupported yet */
                    .and_then(|&ipv4_first| {
                        Some(ipv4_first)
                    })
            })
            .wait();

        let resolved_ip = response
            .and_then(|ipv4_resolved| {
                Ok(ipv4_resolved)
            })
            .unwrap_or_else(|_| Some(ip_localhost));

        info!("Domain: {} resolves to IPv4: {}", domain.cyan(), resolved_ip.unwrap_or(ip_localhost).to_string().cyan());
        Ok(resolved_ip.unwrap())
    }


}
