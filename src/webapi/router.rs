use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::handler::assets::*;


use api::*;
use webapi::internals::*;
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

        route
            .get("/")
            .to_file(format!("{}/web/static/html/panel.html", PROJECT_DIRECTORY));

        route
            .get("/*")
            .to_dir(FileOptions::new(format!("{}/web/", PROJECT_DIRECTORY))
            .with_gzip(true)
            .build()
        );

        route
            .get("/static/*")
            .to_dir(FileOptions::new(format!("{}/web/static/", PROJECT_DIRECTORY))
            .with_gzip(false)
            .build()
        );


        /* WebAPI: */

        // …/version
        route
            .associate(
                &format!("/version"), |handler| {
                    handler.get().to(api_version_get_handler);
                });

        // …/systat
        route
            .associate(
                &format!("/systat"), |handler| {
                    handler.get().to(api_systat_get_handler);
                });

        // …/status/:cell
        route
            .associate(
                &format!("{}:cell", STATUS_RESOURCE), |handler| {
                    handler.get().to(cell_status_get_handler);
                });

        // …/cells/list
        route
            .associate(
                &format!("{}list", CELLS_RESOURCE), |handler| {
                    handler.get().to(cells_get_handler);
                });

        // …/cell/:cell
        route
            .associate(
                &format!("{}:cell", CELL_RESOURCE), |handler| {
                    handler.get().to(cell_get_handler);
                    handler.post().to(cell_post_handler);
                    handler.delete().to(cell_delete_handler);
                });

        // …/snapshot/list/:cell
        route
            .associate(
                &format!("{}list/:cell", SNAPSHOT_RESOURCE), |handler| {
                    handler.get().to(zfs_snapshot_list_handler);
                });

        // …/snapshot/:cell/:snapshot
        route
            .associate(
                &format!("{}:cell/:snapshot", SNAPSHOT_RESOURCE), |handler| {
                    handler.get().to(zfs_snapshot_get_handler);
                    handler.post().to(zfs_snapshot_post_handler);
                    handler.delete().to(zfs_snapshot_delete_handler);
                });

        // …/rollback/:cell/:snapshot
        route
            .associate(
                &format!("{}:cell/:snapshot", ROLLBACK_RESOURCE), |handler| {
                    handler.post().to(zfs_rollback_post_handler);
                });

        // …/datasets/list/:cell
        route
            .associate(
                &format!("{}list/:cell", DATASETS_RESOURCE), |handler| {
                    handler.get().to(zfs_dataset_list_handler);
                });

        // …/proxy/:cell/:from/:to
        route
            .associate(
                &format!("{}:cell/:from/:to", PROXY_RESOURCE), |handler| {
                    handler.post().to(web_proxy_post_handler);
                    handler.delete().to(web_proxy_delete_handler);
                });

        // …/proxies/list
        route
            .associate(
                &format!("{}:list", PROXIES_RESOURCE), |handler| {
                    handler.get().to(web_proxies_get_handler);
                });
    })
}
