use gotham::state::State;
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use gotham::helpers::http::response::create_response;
use mime::*;
use std::thread;
use std::time::Duration;
use chrono::{Local, DateTime, Utc};
use systemstat::*;
use colored::Colorize;

use crate::*;
use crate::apis::status::*;
use crate::apis::cell::*;


/// List Mounts type alias
pub type ListMounts = Vec<SystatMount>;

/// List Network Interfaces type alias
pub type ListNetifs = Vec<SystatNetif>;


/// System Stat
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Systat {

    /// Average System Load
    pub loadavg: Option<SystatSysLoad>,

    /// Uptime in seconds
    pub uptime: Option<u64>,

    /// Boot Time - DateTime String with RFC2822 format
    pub boot_time: Option<String>,

    /// CPU Usage
    pub cpu: Option<SystatCPU>,

    /// Memory Usage
    pub memory: Option<SystatMemory>,

    /// Mounted filesystems
    pub mounts: Option<ListMounts>,

    /// Active Networks
    pub networks: Option<ListNetifs>,

    /// Ressident Processes list:
    pub processes: Option<CellProcesses>,

}


/// System Mounts Stat
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystatMount {

    /// Mounted From
    pub fs_mounted_from: Option<String>,

    /// Mounted Filesystem Type
    pub fs_type: Option<String>,

    /// Mounted On
    pub fs_mounted_on: Option<String>,

    /// Mount Avail
    pub avail: Option<String>,

    /// Mount Total
    pub total: Option<String>,

}


/// System Load Stat
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SystatSysLoad {

    /// 1 Minute Load
    pub one: Option<f64>,

    /// 5 Minutes Load
    pub five: Option<f64>,

    /// 15 Minutes Load
    pub fifteen: Option<f64>,

}


/// System Netif Stat
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystatNetif {

    /// Name of interface
    pub name: Option<String>,

    /// Addresses of interface
    pub addrs: Option<List>,

}


/// CPU Stat
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SystatCPU {

    /// CPU User
    pub user: Option<f64>,

    /// CPU System
    pub system: Option<f64>,

    /// CPU Interrupts
    pub interrupt: Option<f64>,

    /// CPU Idle
    pub idle: Option<f64>,

    /// CPU Temperature
    pub temperature: Option<f64>,

}


/// Memory Stat
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SystatMemory {

    /// Memory Total
    pub total: Option<usize>,

    /// Memory Used
    pub used: Option<usize>,

    /// Memory Free
    pub free: Option<usize>,

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


lazy_static! {

    /// System handle for Systat
    pub static ref SYSTEM: System = System::new();

}


impl Default for Systat {
    fn default() -> Systat {
        thread::spawn(
            move || {
                let mounts_stat = SYSTEM
                    .mounts()
                    .and_then(|mounts| {
                        mounts
                            .iter()
                            .filter(|mount| {
                                mount.fs_type == "zfs"
                                && mount.fs_mounted_from != "zroot"
                                && mount.fs_mounted_from != "zroot/ROOT"
                                && ! mount // filter out all Prison datasets from Systat
                                        .fs_mounted_on
                                        .contains(PRISON_PATH)
                            })
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
                        warn!("Mounts: Failure: {}", err.to_string().cyan());
                        err
                    })
                    .unwrap_or_else(|_| vec!());

                let networks_stat = SYSTEM
                    .networks()
                    .and_then(|orig_netifs| {
                        orig_netifs
                            .values()
                            .filter(|netif| {
                                ! netif.name.starts_with(CELL_NET_INTERFACE)
                            })
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
                                                "".to_string()
                                            },
                                            IpAddr::Empty | IpAddr::Unsupported => {
                                                "".to_string()
                                            },
                                        }
                                    })
                                    .filter(|ref nif| ! nif.is_empty())
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
                    .unwrap_or_else(|_| vec!());

                let memory_stat = SYSTEM
                    .memory()
                    .and_then(|mem| {
                        debug!("Memory total: {}. Memory used: {}. Memory free: {}",
                                mem.total, mem.total - mem.free, mem.free);
                        Ok(
                            SystatMemory {
                                total: Some(mem.total.as_usize()),
                                used: Some((mem.total - mem.free).as_usize()),
                                free: Some(mem.free.as_usize()),
                            }
                        )
                    })
                    .map_err(|err| {
                        warn!("Memory: Failure: {}", err.to_string().red());
                        err
                    })
                    .unwrap_or_default();

                let loadavg_stat = SYSTEM
                    .load_average()
                    .and_then(|loadavg| {
                        debug!("Load average: 1min: {},  5min: {},  15min: {}",
                               loadavg.one.to_string().cyan(), loadavg.five.to_string().cyan(), loadavg.fifteen.to_string().cyan());
                        Ok(
                            SystatSysLoad {
                                one: Some(loadavg.one.into()),
                                five: Some(loadavg.five.into()),
                                fifteen: Some(loadavg.fifteen.into()),
                            }
                        )
                    })
                    .map_err(|err| {
                        warn!("Load average: Failure: {}", err.to_string().red());
                        err
                    })
                    .unwrap_or_default();

                let uptime_stat = SYSTEM
                    .uptime()
                    .and_then(|duration| {
                        debug!("Uptime: {}s", duration.as_secs().to_string().cyan());
                        Ok(
                           duration.as_secs()
                        )
                    })
                    .map_err(|err| {
                        warn!("Uptime: Failure: {}", err.to_string().red());
                        err
                    })
                    .unwrap_or(0);

                let utc_now = Local::now().naive_local();
                let rfc_date_now = DateTime::<Utc>::from_utc(utc_now, Utc).to_rfc2822();
                let boottime_stat = SYSTEM
                    .boot_time()
                    .and_then(|boot_time| {
                        let rfc_date = boot_time.to_rfc2822();
                        debug!("BootTime: {}", rfc_date.to_string().cyan());
                        Ok(
                           rfc_date
                        )
                    })
                    .map_err(|err| {
                        warn!("BootTime: Failure: {}", err.to_string().red());
                        err
                    })
                    .unwrap_or(rfc_date_now);

                let cputemp_stat = SYSTEM
                    .cpu_temp()
                    .and_then(|cpu_temp| {
                        debug!("CPU Temperature: {}", cpu_temp.to_string().cyan());
                        Ok(cpu_temp)
                    })
                    .map_err(|err| {
                        warn!("CPU Temperature Failure: {}", err.to_string().red())
                    })
                    .unwrap_or(0.0);

                let cpu_stat = SYSTEM
                    .cpu_load_aggregate()
                    .and_then(|main_cpu| {
                        thread::sleep(Duration::from_millis(SYSTAT_CPUSTAT_INTERVAL));
                        main_cpu
                            .done()
                            .and_then(|cpu| {
                                debug!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                                    cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
                                Ok(
                                    SystatCPU { // NOTE: percentage:
                                        user: Some(100.0 * f64::from(cpu.user)),
                                        system: Some(100.0 * f64::from(cpu.system)),
                                        interrupt: Some(100.0 * f64::from(cpu.interrupt)),
                                        idle: Some(100.0 * f64::from(cpu.idle)),
                                        temperature: Some(cputemp_stat.into()),
                                    }
                                )
                            })
                    })
                    .map_err(|err| {
                        warn!("CPU load: Failure: {}", err.to_string().red());
                        err
                    })
                    .unwrap_or_default();

                let superuser_processes = CellProcesses::of_uid(0)
                    .and_then(|processes| {
                        Ok(processes)
                    })
                    .unwrap_or_default();

                // Now wrap everything with a single structure:
                Systat {
                    loadavg: Some(loadavg_stat),
                    uptime: Some(uptime_stat),
                    boot_time: Some(boottime_stat),
                    cpu: Some(cpu_stat),
                    memory: Some(memory_stat),
                    mounts: Some(mounts_stat),
                    networks: Some(networks_stat),
                    processes: Some(superuser_processes),
                }
            }
        )
        .join()
        .unwrap_or_default()
    }
}


/// Serialize to JSON on .to_string()
impl ToString for Systat {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}"))
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
                .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}")),
        )
    }
}
