use domain::bits::name::FromStrError;
use std::net::{IpAddr, Ipv4Addr};
use abstract_ns::HostResolve;
use ns_std_threaded::ThreadedResolver;
use futures::future::*;


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
    pub fn validate_domain_addresses(from: &String, to: &String) -> Result<(IpAddr, IpAddr), FromStrError> {
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
                                                  valid_ipv4_from, valid_ipv4_to, validate_state, validate_state2);
                            error!("{}", err_msg);
                            Err(FromStrError::EmptyLabel)
                        } else {
                            debug!("validate_domain_addresses(): IPv4 pair: {} -> {}", valid_ipv4_from, valid_ipv4_to);
                            Ok((valid_ipv4_from, valid_ipv4_to))
                        }
                    })
            })
    }


    /// Asynchronously resolves first available IPv4 defined for given "domain":
    pub fn lookup_domain(domain: &String) -> Result<IpAddr, FromStrError> {
        let ip_localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)); // Used as fallback if domain resolve fails
        let response = ThreadedResolver::new()
            .resolve_host(&domain.parse().unwrap())
            .map(|addresses| {
                debug!("Domain: {} resolves to IPv4(s): [{:?}]", &domain, &addresses);
                addresses
                    .iter()
                    .filter(|&element| !element.to_string().contains(":")) /* NOTE: filter out IPv6 - unsupported yet */
                    .next()
                    .and_then(|&ipv4_first| {
                        Some(ipv4_first)
                    })
            })
            .wait();

        let resolved_ip = response
            .and_then(|ipv4_resolved| {
                Ok(ipv4_resolved)
            })
            .unwrap_or(Some(ip_localhost));

        info!("Domain: {} resolves to IPv4: {}", domain, resolved_ip.unwrap());
        Ok(resolved_ip.unwrap())
    }


}
