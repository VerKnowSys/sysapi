
/* Defaults */
pub const GVR_BIN: &'static str = "gvr";
pub const JAIL_BIN: &'static str = "jail";
pub const DEFAULT_ADDRESS: &'static str = "172.16.3.1:80";
pub const CELLS_PATH: &'static str = "/Shared/Prison/Cells";


/* API */
pub const CELL_RESOURCE: &'static str = "/cell/";
pub const SENTRY_RESOURCE: &'static str = "/sentry/";
pub const IGNITER_RESOURCE: &'static str = "/igniter/";
pub const ZONE_RESOURCE: &'static str = "/zone/";
pub const PROXY_RESOURCE: &'static str = "/proxy/";
pub const STATUS_RESOURCE: &'static str = "/status/";
pub const SNAPSHOT_RESOURCE: &'static str = "/snapshot/";
pub const ROLLBACK_RESOURCE: &'static str = "/rollback/";


/// Internal SysAPI:
pub mod api;

/// Internal CellAPI:
pub mod cell;
