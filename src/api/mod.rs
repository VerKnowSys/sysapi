
/* Defaults */
pub const GVR_BIN: &'static str = "gvr";
pub const ZFS_BIN: &'static str = "zfs";
pub const JAIL_BIN: &'static str = "jail";
pub const DEFAULT_ADDRESS: &'static str = "172.16.3.1:80";
pub const CELLS_PATH: &'static str = "/Shared/Prison/Cells";
pub const SENTRY_PATH: &'static str = "/Shared/Prison/Sentry";


/* API */
pub const CELL_RESOURCE: &'static str = "/cell/";
pub const CELLS_RESOURCE: &'static str = "/cells/";
pub const SENTRY_RESOURCE: &'static str = "/sentry/";
pub const IGNITER_RESOURCE: &'static str = "/igniter/";
pub const ZONE_RESOURCE: &'static str = "/zone/";
pub const PROXY_RESOURCE: &'static str = "/proxy/";
pub const STATUS_RESOURCE: &'static str = "/status/";
pub const SNAPSHOT_RESOURCE: &'static str = "/snapshot/";
pub const ROLLBACK_RESOURCE: &'static str = "/rollback/";


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
