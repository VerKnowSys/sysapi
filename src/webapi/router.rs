use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};


use api::*;
use webapi::cells::*;


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

        route.associate(
            &format!("{resource}:cell", resource = CELL_RESOURCE), |handler| {
                handler.get().to(get_handler);
                handler.post().to(post_handler);
                handler.delete().to(delete_handler);
            });

        route.associate(
            &format!("{resource}:cell", resource = SENTRY_RESOURCE), |handler| {
                handler.get().to(get_handler);
                handler.post().to(post_handler);
                handler.delete().to(delete_handler);
            })

    })
}
