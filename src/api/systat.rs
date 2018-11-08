use gotham::state::State;
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use gotham::helpers::http::response::create_response;
use mime::*;
use std::time::Duration;
use chrono::Timelike;
use std::thread; // XXX: temporary
use systemstat::*;
use systemstat::ByteSize;


use api::cell::List;


/// List Mounts type alias
pub type ListMounts = Vec<SystatMount>;

/// List Network Interfaces type alias
pub type ListNetifs = Vec<SystatNetif>;


/// System Stat
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Systat {

    /// Average System Load
    loadavg: Option<SystatSysLoad>,

    /// Uptime in seconds
    uptime: Option<u64>,

    /// Boot Time in seconds
    boot_time: Option<u64>,

    /// CPU Usage
    cpu: Option<SystatCPU>,

    /// Memory Usage
    memory: Option<SystatMemory>,

    /// Mounted filesystems
    mounts: Option<ListMounts>,

    /// Active Networks
    networks: Option<ListNetifs>,

    /// Active Network Statistics
    network_stats: Option<SystatNetstats>,

}


/// System SystatNetstats Stat
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SystatNetstats {

    /// tcp_sockets in use
    tcp_sockets_in_use: Option<usize>,

    /// tcp_sockets_o ph ned
    tcp_sockets_orphaned: Option<usize>,

    /// udp_sockets in use
    udp_sockets_in_use: Option<usize>,

    /// tcp6_sockets in use
    tcp6_sockets_in_use: Option<usize>,

    /// udp6_sockets in use
    udp6_sockets_in_use: Option<usize>,

}


/// System Mounts Stat
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystatMount {

    /// Mounted From
    fs_mounted_from: Option<String>,

    /// Mounted Filesystem Type
    fs_type: Option<String>,

    /// Mounted On
    fs_mounted_on: Option<String>,

    /// Mount Avail
    avail: Option<String>,

    /// Mount Total
    total: Option<String>,

}


/// System Load Stat
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SystatSysLoad {

    /// 1 Minute Load
    one: Option<f64>,

    /// 5 Minutes Load
    five: Option<f64>,

    /// 15 Minutes Load
    fifteen: Option<f64>,

}


/// System Netif Stat
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystatNetif {

    /// Name of interface
    name: Option<String>,

    /// Addresses of interface
    addrs: Option<List>,

}


/// CPU Stat
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SystatCPU {

    /// CPU User
    user: Option<f64>,

    /// CPU System
    system: Option<f64>,

    /// CPU Interrupts
    interrupt: Option<f64>,

    /// CPU Idle
    idle: Option<f64>,

    /// CPU Temperature
    temperature: Option<f64>,

}


/// Memory Stat
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SystatMemory {

    /// Memory Total
    total: Option<usize>,

    /// Memory Used
    used: Option<usize>,

    /// Memory Free
    free: Option<usize>,

}


impl Default for SystatMemory {
    fn default() -> SystatMemory {
        SystatMemory {
            total: None,
            used: None,
            free: None,
        }
    }
}


impl Default for SystatSysLoad {
    fn default() -> SystatSysLoad {
        SystatSysLoad {
            one: None,
            five: None,
            fifteen: None,
        }
    }
}


impl Default for SystatCPU {
    fn default() -> SystatCPU {
        SystatCPU {
            user: None,
            system: None,
            interrupt: None,
            idle: None,
            temperature: None,
        }
    }
}


impl Default for SystatNetstats {
    fn default() -> SystatNetstats {
        SystatNetstats {
            tcp_sockets_in_use: None,
            tcp_sockets_orphaned: None,
            udp_sockets_in_use: None,
            tcp6_sockets_in_use: None,
            udp6_sockets_in_use: None,
        }
    }
}


impl Default for Systat {
    fn default() -> Systat {
        let system = System::new();

        let mounts_stat = system
            .mounts()
            .and_then(|mounts| {
                mounts
                    .iter()
                    .map(|mount| {
                        Ok(
                            SystatMount {
                                fs_mounted_from: Some(mount.fs_mounted_from.to_string()),
                                fs_type: Some(mount.fs_type.to_string()),
                                fs_mounted_on: Some(mount.fs_mounted_on.to_string()),
                                avail: Some(mount.avail.to_string(true)),
                                total: Some(mount.total.to_string(true)),
                            }
                        )
                    })
                    .collect()
            })
            .map_err(|err| {
                warn!("Mounts: Failure: {}", err);
                err
            })
            .unwrap_or(vec!());

        let networks_stat = system
            .networks()
            .and_then(|orig_netifs| {
                orig_netifs
                    .values()
                    .map(|netif| {
                        let addrs = netif
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
                            .collect::<List>();
                        Ok(
                            SystatNetif {
                                name: Some(netif.name.to_string()),
                                addrs: Some(addrs),
                            }
                        )
                    })
                    .collect()
            })
            .unwrap_or(vec!());

        let memory_stat = system
            .memory()
            .and_then(|mem| {
                debug!("Memory total: {}. Memory used: {}. Memory free: {}",
                        mem.total, mem.total - mem.free, mem.free);
                Ok(
                    SystatMemory {
                        total: Some(ByteSize::from(mem.total).as_usize()),
                        used: Some(ByteSize::from(mem.total - mem.free).as_usize()),
                        free: Some(ByteSize::from(mem.free).as_usize()),
                    }
                )
            })
            .map_err(|err| {
                warn!("Memory: Failure: {}", err);
                err
            })
            .unwrap_or(SystatMemory::default());

        let loadavg_stat = system
            .load_average()
            .and_then(|loadavg| {
                debug!("Load average: 1min: {},  5min: {},  15min: {}",
                       loadavg.one, loadavg.five, loadavg.fifteen);
                Ok(
                    SystatSysLoad {
                        one: Some(loadavg.one.into()),
                        five: Some(loadavg.five.into()),
                        fifteen: Some(loadavg.fifteen.into()),
                    }
                )
            })
            .map_err(|err| {
                warn!("Load average: Failure: {}", err);
                err
            })
            .unwrap_or(SystatSysLoad::default());

        let uptime_stat = system
            .uptime()
            .and_then(|uptime| {
                let duration = Duration::from(uptime);
                debug!("Uptime: {}s", duration.as_secs());
                Ok(
                   duration.as_secs()
                )
            })
            .map_err(|err| {
                warn!("Uptime: Failure: {}", err);
                err
            })
            .unwrap_or(0);

        let boottime_stat = system
            .boot_time()
            .and_then(|boot_time| {
                let duration = DateTime::from(boot_time);
                debug!("BootTime: {}s", duration.second());
                Ok(
                   duration.second()
                )
            })
            .map_err(|err| {
                warn!("BootTime: Failure: {}", err);
                err
            })
            .unwrap_or(0);

        let cputemp_stat = system
            .cpu_temp()
            .and_then(|cpu_temp| {
                debug!("CPU Temperature: {}", cpu_temp);
                Ok(cpu_temp)
            })
            .map_err(|err| {
                warn!("CPU Temperature Failure: {}", err)
            })
            .unwrap_or(0.0);

        let cpu_stat = system
            .cpu_load_aggregate()
            .and_then(|main_cpu| {
                debug!("CPU Load - Measure in progress…");
                thread::sleep(Duration::from_secs(1)); // XXX: TODO: make a future from it and process async timeout:
                main_cpu
                    .done()
                    .and_then(|cpu| {
                        debug!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                            cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
                        Ok(
                            SystatCPU {
                                user: Some(cpu.user.into()),
                                system: Some(cpu.system.into()),
                                interrupt: Some(cpu.interrupt.into()),
                                idle: Some(cpu.idle.into()),
                                temperature: Some(cputemp_stat.into()),
                            }
                        )
                    })
            })
            .map_err(|err| {
                warn!("CPU load: Failure: {}", err);
                err
            })
            .unwrap_or(SystatCPU::default());

        let network_stats = system
            .socket_stats()
            .and_then(|stats| {
                debug!("System socket statistics: {:?}", stats);
                Ok(
                    SystatNetstats {
                       tcp_sockets_in_use: Some(stats.tcp_sockets_in_use),
                       tcp_sockets_orphaned: Some(stats.tcp_sockets_orphaned),
                       udp_sockets_in_use: Some(stats.udp_sockets_in_use),
                       tcp6_sockets_in_use: Some(stats.tcp6_sockets_in_use),
                       udp6_sockets_in_use: Some(stats.udp6_sockets_in_use),
                   }
                )
            })
            .map_err(|err| {
                warn!("Netstats Failure: {}", err);
                err
            })
            .unwrap_or(SystatNetstats::default());

        // Now wrap everything with a single structure:
        Systat {
            loadavg: Some(loadavg_stat),
            uptime: Some(uptime_stat),
            boot_time: Some(boottime_stat.into()),
            cpu: Some(cpu_stat),
            memory: Some(memory_stat),
            mounts: Some(mounts_stat),
            networks: Some(networks_stat),
            network_stats: Some(network_stats),
        }
    }
}


/// Serialize to JSON on .to_string()
impl ToString for Systat {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


/// Implement response for GETs:
impl IntoResponse for Systat {
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
