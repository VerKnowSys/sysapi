use std::path::Path;
use gotham::state::State;
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use gotham::helpers::http::response::create_response;
use std::io::BufReader;
use std::fs;
use mime::*;

use std::net::*;
use std::io::{Write, Error, ErrorKind};
use atomicwrites::{AtomicFile,AllowOverwrite};


use api::*;
use zone::*;
use webapi::utils::list_proxies;


use regex::Regex;
lazy_static! {
    /// Regex to extract cell name from proxy file name:
    pub static ref CELL_NAME_FROM_FILE_NAME_PATTERN: Regex = {
        Regex::new(r"(?:([a-zA-Z0-9]+))_.*").unwrap()
    };

    /// Regex to extract proxy from
    pub static ref PROXYFROM_FROM_FILE_NAME_PATTERN: Regex = {
        Regex::new(r"proxyfrom_(?:([a-zA-Z0-9_-]+))_proxyto.*").unwrap()
    };

    /// Regex to extract proxy to
    pub static ref PROXYTO_FROM_FILE_NAME_PATTERN: Regex = {
        Regex::new(r"proxyto_(?:([a-zA-Z0-9_-]+))\..*").unwrap()
    };

}


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


/// Proxies (Proxy List) structure for easy list management
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proxies {

    /// List of all proxies
    pub list: Vec<Proxy>

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


impl Default for Proxies {
    fn default() -> Proxies {
        Proxies {
            list:
                list_proxies()
                    .iter()
                    .flat_map(|proxy_file_name| {
                        // Infer data from proxy file name:
                        let cell_name = &CELL_NAME_FROM_FILE_NAME_PATTERN
                            .captures(&proxy_file_name)
                            .and_then(|cap| {
                                match cap.get(1).map_or("", |m| m.as_str()) {
                                    "" => None,
                                    domain => Some(domain),
                                }
                            });
                        let proxy_from = &PROXYFROM_FROM_FILE_NAME_PATTERN
                            .captures(&proxy_file_name)
                            .and_then(|cap| {
                                match cap.get(1).map_or("", |m| m.as_str()) {
                                    "" => None,
                                    domain => Some(domain),
                                }
                            });
                        let proxy_to = &PROXYTO_FROM_FILE_NAME_PATTERN
                            .captures(&proxy_file_name)
                            .and_then(|cap| {
                                match cap.get(1).map_or("", |m| m.as_str()) {
                                    "" => None,
                                    domain => Some(domain),
                                }
                            });
                        let name = cell_name.unwrap_or("");
                        let from = proxy_from.unwrap_or("").replace("_", ".");
                        let to = proxy_to.unwrap_or("").replace("_", ".");
                        debug!("Proxy for cell: {}. Proxy from: {}. Proxy to: {}", name, from, to);

                        Proxy::new(&name.to_string(), &from.to_string(), &to.to_string())
                    })
                    .collect()
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


    /// Generate proxy entry (validation pass for: from/to is required)
    pub fn new(cell_name: &String, from: &String, to: &String) -> Result<Proxy, Error> {
        Zone::validate_domain_addresses(from, to)
            .and_then(|(valid_ipv4_from, valid_ipv4_to)| {
                // When both domains are valid, create Proxy object:
                Ok(Proxy {
                    cell: Some(cell_name.to_string()),
                    config: Some(Proxy::config_from_template(from, to)),
                    from: Some(from.to_string()),
                    from_ipv4: Some(valid_ipv4_from),
                    to: Some(to.to_string()),
                    to_ipv4: Some(valid_ipv4_to),
                })
            })
            .map_err(|err| {
                error!("{}", err);
                Error::new(ErrorKind::Other, err)
            })
    }


    /// Create new Web Proxy configuration:
    pub fn create(cell_name: &String, from: &String, to: &String) -> Result<Proxy, Error> {
        Proxy::new(cell_name, from, to)
            .and_then(|proxy| {
                // Write Nginx proxy object to local file under dir: /Shared/Prison/Sentry/CELLNAME/cell-webconfs/*.conf:
                let proxy_file_name = format!("{}_proxyfrom_{}_proxyto_{}.conf", cell_name, from.replace(".", "_"), to.replace(".", "_"));
                let proxy_dest_dir = format!("{}/{}/cell-webconfs", SENTRY_PATH, cell_name);
                let proxy_dest_file = format!("{}/{}", proxy_dest_dir, proxy_file_name);

                AtomicFile::new(proxy_dest_file.clone(), AllowOverwrite)
                    .write(|file| {
                        file.write_all(
                            proxy.clone()
                                .config.unwrap()
                                .to_string()
                                .as_bytes()
                        )
                    })
                    .and_then(|_| {
                        info!("Proxy: Written Web-Proxy config to file: {}. Which belongs to cell: {}",
                              &proxy_dest_file, &cell_name);
                        // Finally - return object metadata- since atomic write succeded at this point:
                        Ok(proxy)
                    })
                    .map_err(|err| {
                        let err_msg = format!("Atomic Write Failed for file: {}. Error details: {:?}!",
                                              &proxy_dest_file, err);
                        error!("{}", err_msg);
                        Error::new(ErrorKind::Other, err_msg)
                    })
            })
            .map_err(|err| {
                error!("{}", err);
                Error::new(ErrorKind::Other, err)
            })
    }


    /// Destroy Web Proxy configuration:
    pub fn destroy(cell_name: &String, from: &String, to: &String) -> Result<(), Error> {
        let proxy_file_name = format!("{}_proxyfrom_{}_proxyto_{}.conf", cell_name, from.replace(".", "_"), to.replace(".", "_"));
        let proxy_dest_dir = format!("{}/{}/cell-webconfs", SENTRY_PATH, cell_name);
        let proxy_dest_file = format!("{}/{}", proxy_dest_dir, proxy_file_name);
        debug!("Calling Proxy::destroy() on file: {}", &proxy_dest_file);

        match fs::remove_file(&proxy_dest_file) {
            Ok(_) => {
                debug!("Destroyed Proxy configuration from file: {}", &proxy_dest_file);
                Ok(())
            },
            Err(err) => {
                let err_msg = format!("Error destroying Proxy file: {}! Error details: {:?}", &proxy_dest_file, err);
                error!("{}", err_msg);
                Err(Error::new(ErrorKind::Other, err_msg))
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
    set $backend 'https://{}';

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


/// Implement response for GETs:
impl IntoResponse for Proxy {
    fn into_response(self, state: &State) -> Response<Body> {
        // serialize only if name is set - so Proxy is initialized/ exists
        match self.from {
            Some(_) =>
                create_response(
                    state,
                    StatusCode::OK,
                    APPLICATION_JSON,
                    serde_json::to_string(&self)
                        .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}")),
                ),
            None =>
                create_response(
                    state,
                    StatusCode::NOT_FOUND,
                    APPLICATION_JSON,
                    Body::from("{\"status\": \"NotFound\"}"),
                )
        }
    }
}


/// Implement response for GETs:
impl IntoResponse for Proxies {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            APPLICATION_JSON,
            serde_json::to_string(&self)
                .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}")),
        )
    }
}
