use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};


use api::*;
use webapi::cells::*;
use webapi::datasets::*;


/// Define router
pub fn router() -> Router {
    // TODO: define routes
    // const IGNITER_RESOURCE: &'static str = "/igniter/";
    // const ZONE_RESOURCE: &'static str = "/zone/";
    // const PROXY_RESOURCE: &'static str = "/proxy/";
    // const STATUS_RESOURCE: &'static str = "/status/";
    // const SNAPSHOT_RESOURCE: &'static str = "/snapshot/";
    // const ROLLBACK_RESOURCE: &'static str = "/rollback/";


    build_simple_router(|route| {

        // …/cells/list
        route.associate(
            &format!("{}list", CELLS_RESOURCE), |handler| {
                handler.get().to(cells_get_handler);
            });

        // …/cell/:cell
        route.associate(
            &format!("{}:cell", CELL_RESOURCE), |handler| {
                handler.get().to(cell_get_handler);
                handler.post().to(cell_post_handler);
                handler.delete().to(cell_delete_handler);
            });

        // …/snapshot/list/:cell
        route.associate(
            &format!("{}list/:cell", SNAPSHOT_RESOURCE), |handler| {
                handler.get().to(zfs_snapshot_list_handler);
            });

        // …/snapshot/:cell/:snapshot
        route.associate(
            &format!("{}:cell/:snapshot", SNAPSHOT_RESOURCE), |handler| {
                handler.get().to(zfs_snapshot_get_handler);
                handler.post().to(zfs_snapshot_post_handler);
                handler.delete().to(zfs_snapshot_delete_handler);
            });

        // …/rollback/:cell/:snapshot
        route.associate(
            &format!("{}:cell/:snapshot", ROLLBACK_RESOURCE), |handler| {
                handler.post().to(zfs_rollback_post_handler);
            });

    })
}
