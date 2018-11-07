use gotham::state::State;
use std::thread;
use std::time::Duration;
use systemstat::{System, Platform};
use std::collections::HashMap;
use systemstat::*;


use api::cell::List;


/// handle GET for /version
pub fn api_version_get_handler(state: State) -> (State, String) {
    let api_version = env!("CARGO_PKG_VERSION");
    let formatted_version = format!("{{\"status\": \"OK\", \"version\": \"{}\"}}", api_version);
    (state, formatted_version)
}


/// handle GET for /systat
pub fn api_systat_get_handler(state: State) -> (State, String) {
    let sys = System::new();
    let mut systat = HashMap::new();

    match sys.mounts() {
        Ok(mounts) => {
            let all: List = mounts
                .iter()
                .map(|mount| {
                    format!("{}", mount.fs_mounted_on) // TODO: wrap it around structures
                })
                .collect();
            systat.insert(
                "Mounted Filesystems".to_string(),
                format!("{}", all.join(" ")),
            );
        }
        Err(x) => warn!("Mounts: error: {}", x)
    }

    match sys.block_device_statistics() {
        Ok(stats) => {
            let all: List = stats
                .values()
                .map(|stat| {
                    format!("{}='{:?}'", stat.name, stat) // TODO: wrap it around structures
                })
                .collect();
            systat.insert(
                "Block Statistics".to_string(),
                format!("{}", all.join(" ")),
            );
        }
        Err(x) => warn!("Block statistics error: {}", x.to_string())
    }

    match sys.networks() {
        Ok(orig_netifs) => {
            let netifs: List = orig_netifs
                .values()
                .map(|netif| {
                    format!("{}", netif.name)
                })
                .filter(|nif| !nif.is_empty())
                .collect();
            systat.insert(
                "Network Interfaces".to_string(),
                format!("{}", netifs.join(" ")),
            );
            let netaddrs: List = orig_netifs
                .values()
                .flat_map(|netif| {
                    netif
                        .addrs
                        .iter()
                        .map(|nif| {
                            match nif.addr {
                                IpAddr::V4(addr) => {
                                    format!("{}", addr)
                                },
                                IpAddr::V6(_addr) => {
                                    // NOTE: Skip IPv6 format!("{}", addr)
                                    format!("")
                                },
                                IpAddr::Empty | IpAddr::Unsupported => {
                                    format!("")
                                },
                            }
                        })
                        .filter(|nif| !nif.is_empty())
                        .collect::<List>()
                })
                .collect();
            systat.insert(
                "Network Addresses".to_string(),
                format!("{}", netaddrs.join(" ")),
            );
        }
        Err(x) => warn!("Networks: error: {}", x)
    }

    match sys.networks() {
        Ok(netifs) => {
            let netifs: List = netifs
                .values()
                .map(|netif| {
                    match sys.network_stats(&netif.name) {
                        Ok(nif) => format!("{}='{:?}'", netif.name, nif),
                        Err(_) => format!("{}", netif.name)
                    }
                })
                .filter(|nif| !nif.is_empty())
                .collect();
            systat.insert(
                "Network Stats".to_string(),
                format!("{}", netifs.join(" ")),
            );
        }
        Err(x) => warn!("Networks: error: {}", x)
    }

    match sys.memory() {
        Ok(mem) => {
            debug!("Memory: {} used / {} ({} bytes) total ({:?})", mem.total - mem.free, mem.total, mem.total.as_usize(), mem.platform_memory);
            systat.insert(
                "Memory Total".to_string(),
                format!("{}", mem.total),
            );
            systat.insert(
                "Memory Used".to_string(),
                format!("{}", mem.total - mem.free),
            );
            systat.insert(
                "Memory Free".to_string(),
                format!("{}", mem.free),
            );
            systat.insert(
                "Memory Max".to_string(),
                format!("'{:?}'", mem.platform_memory),
            );
        },
        Err(x) => warn!("Memory: error: {}", x)
    }

    match sys.load_average() {
        Ok(loadavg) => {
            debug!("Load average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen);
            systat.insert(
                "Load Average".to_string(),
                format!("{} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
            );
        },
        Err(x) => warn!("Load average: error: {}", x)
    }

    match sys.uptime() {
        Ok(uptime) => {
            debug!("Uptime: {:?}", uptime);
            systat.insert(
                "Uptime".to_string(),
                format!("'{:?}'", uptime),
            );
        },
        Err(x) => warn!("Uptime: error: {}", x)
    }

    match sys.boot_time() {
        Ok(boot_time) => {
            debug!("Boot time: {}", boot_time);
            systat.insert(
                "Boot Time".to_string(),
                format!("{}", boot_time),
            );
        },
        Err(x) => warn!("Boot time: error: {}", x)
    }

    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            debug!("Measuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            debug!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);

            systat.insert(
                "CPU User".to_string(),
                format!("{}%", cpu.user * 100.0),
            );
            systat.insert(
                "CPU System".to_string(),
                format!("{}%", cpu.system * 100.0),
            );
            systat.insert(
                "CPU Interrupts".to_string(),
                format!("{}%", cpu.interrupt * 100.0),
            );
            systat.insert(
                "CPU Idle".to_string(),
                format!("{}%", cpu.idle * 100.0),
            );
        },
        Err(x) => warn!("CPU load: error: {}", x)
    }

    match sys.cpu_temp() {
        Ok(cpu_temp) => {
            debug!("CPU temp: {}", cpu_temp);
            systat.insert(
                "CPU Temperature".to_string(),
                format!("{}", cpu_temp),
            );
        },
        Err(x) => warn!("CPU temp: {}", x)
    }

    match sys.socket_stats() {
        Ok(stats) => {
            debug!("System socket statistics: {:?}", stats);
            systat.insert(
                "Socket Stats".to_string(),
                format!("'{:?}'", stats),
            );
        },
        Err(x) => warn!("Error: {}", x.to_string())
    }

    let list: List = systat
        .iter()
        .map(|(key, value)| format!("\"{}\": \"{}\"", key, value))
        .collect();
    let formatted_systat = format!("{{\"status\": \"OK\", \"systat\": {{ {} }} }}", list.join(", "));
    (state, formatted_systat)
}
