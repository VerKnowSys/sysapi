use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::handler::assets::*;


use api::*;
use webapi::cells::*;
use webapi::datasets::*;
use webapi::proxies::*;


/// Define router
pub fn router() -> Router {
    // TODO: define all missing routes:
    // TODO: const IGNITER_RESOURCE: &'static str = "/igniter/";
    // TODO: const ZONE_RESOURCE: &'static str = "/zone/";
    // TODO: const STATUS_RESOURCE: &'static str = "/status/";


    build_simple_router(|route| {


        /* Dashboard */

        route.get("/").to_file("web/dashboard.html");

        route.get("/css/*").to_dir(
            FileOptions::new("web/static/css/")
                .with_gzip(true)
                .build(),
        );

        route.get("/js/*").to_dir(
            FileOptions::new("web/static/js/")
                .with_gzip(true)
                .build(),
        );

        route.get("/static/*").to_dir(
            FileOptions::new("web/static/")
                .with_gzip(false)
                .build(),
        );


        /* WebAPI: */

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


        // …/proxy/:cell/:from/:to
        route.associate(
            &format!("{}:cell/:from/:to", PROXY_RESOURCE), |handler| {
                handler.post().to(web_proxy_post_handler);
            });
    })
}
