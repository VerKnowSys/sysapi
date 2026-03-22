use rocket::{Route, fs::NamedFile, get, routes};
use std::path::Path;

use crate::{
    processors::{cells::*, datasets::*, proxies::*, systats::*},
    *,
};


/// Index page handler - serves panel.html
#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(PROJECT_DIRECTORY).join("web/static/html/panel.html"))
        .await
        .ok()
}


/// Returns all API routes
pub fn routes() -> Vec<Route> {
    routes![
        index,
        api_version_get_handler,
        api_systat_get_handler,
        cell_status_get_handler,
        cells_get_handler,
        cell_get_handler,
        cell_post_handler,
        cell_delete_handler,
        zfs_snapshot_list_handler,
        zfs_snapshot_get_handler,
        zfs_snapshot_post_handler,
        zfs_snapshot_delete_handler,
        zfs_rollback_post_handler,
        zfs_dataset_list_handler,
        web_proxy_post_handler,
        web_proxy_delete_handler,
        web_proxies_get_handler,
    ]
}
