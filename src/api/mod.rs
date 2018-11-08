/// Project defaults

/// Project directory (for static files access for router):
pub const PROJECT_DIRECTORY: &'static str = "/Projects/sysapi";


/// svdOS cell governor:
pub const GVR_BIN: &'static str = "/usr/bin/gvr";

/// ZFS utility:
pub const ZFS_BIN: &'static str = "/sbin/zfs";

/// BSD jail utility:
pub const JAIL_BIN: &'static str = "/usr/sbin/jail";

/// BSD jail-exec utility:
pub const JEXEC_BIN: &'static str = "/usr/sbin/jexec";

/// Default username (jail user):
pub const CELL_USERNAME: &'static str = "worker";

/// Default local DNS server address:
pub const DEFAULT_DNS: &'static str = "172.16.3.1";

/// Default listen address to listen on:
pub const DEFAULT_ADDRESS: &'static str = "172.16.3.1:80";

/// Default path to Prison root dir:
pub const PRISON_PATH: &'static str = "/Shared/Prison";

/// Default path to cells data dirs:
pub const CELLS_PATH: &'static str = "/Shared/Prison/Cells";

/// Default path to sentry metadata dirs:
pub const SENTRY_PATH: &'static str = "/Shared/Prison/Sentry";


/// WebAPI

/// Cell management:
pub const CELL_RESOURCE: &'static str = "/cell/";

/// Cell lists management:
pub const CELLS_RESOURCE: &'static str = "/cells/";

/// Igniter management:
pub const IGNITER_RESOURCE: &'static str = "/igniter/";

/// DNS zone management:
pub const ZONE_RESOURCE: &'static str = "/zone/";

/// Web proxy management:
pub const PROXY_RESOURCE: &'static str = "/proxy/";

/// Web proxies management:
pub const PROXIES_RESOURCE: &'static str = "/proxies/";

/// Cell status management:
pub const STATUS_RESOURCE: &'static str = "/status/";

/// Cell ZFS Snapshot management:
pub const SNAPSHOT_RESOURCE: &'static str = "/snapshot/";

/// Cell ZFS Rollback management:
pub const ROLLBACK_RESOURCE: &'static str = "/rollback/";

/// Cell ZFS datasets management:
pub const DATASETS_RESOURCE: &'static str = "/datasets/";


/// Internal CellAPI module with system cell management:
pub mod cell;

/// Internal SysAPI module with igniter actions:
pub mod igniter;

/// Internal SysAPI module with web proxy:
pub mod proxy;

/// Internal SysAPI module with system status:
pub mod status;

/// Internal SysAPI module with ZFS features:
pub mod zfs;

/// Internal SysAPI module with local DNS management:
pub mod zone;

/// Cell Systat module:
pub mod systat;
