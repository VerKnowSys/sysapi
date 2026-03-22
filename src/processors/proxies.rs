use rocket::{delete, get, http::Status, post, response::status::Custom, serde::json::Json};

use crate::apis::proxy::*;
use crate::*;


/// Handle POSTs /proxy/<cell>/<from>/<to>
#[post("/proxy/<cell>/<from>/<to>")]
pub fn web_proxy_post_handler(cell: String, from: String, to: String) -> Custom<Json<String>> {
    match Proxy::create(&cell, &from, &to) {
        Ok(result_config) => {
            let res_text = format!(
                "{{\"status\": \"Successfully created new proxy configuration for cell: {}.\"}}",
                cell
            );
            debug!(
                "web_proxy_post_handler(): {}. RESULT-CONFIG: {:?}",
                res_text, result_config
            );
            Custom(Status::Ok, Json(res_text))
        }
        Err(err) => {
            let res_text = format!(
                "{{\"status\": \"Failed to create new proxy configuration for cell: {}. Error details: {}.\"}}",
                cell, err
            );
            error!("{}", res_text);
            Custom(Status::BadRequest, Json(res_text))
        }
    }
}


/// Handle DELETEs /proxy/<cell>/<from>/<to>
#[delete("/proxy/<cell>/<from>/<to>")]
pub fn web_proxy_delete_handler(
    cell: String,
    from: String,
    to: String,
) -> Custom<Json<String>> {
    debug!(
        "web_proxy_delete_handler(): cell: {}, from: {}, to: {}",
        cell, from, to
    );

    match Proxy::destroy(&cell, &from, &to) {
        Ok(_) => {
            let res_text = format!(
                "{{\"status\": \"Successfully destroyed proxy configuration for cell: {}.\"}}",
                cell
            );
            Custom(Status::Ok, Json(res_text))
        }
        Err(err) => {
            let res_text = format!(
                "{{\"status\": \"Failed to destroy proxy configuration for cell: {}. Error details: {}.\"}}",
                cell, err
            );
            error!("{}", res_text);
            Custom(Status::BadRequest, Json(res_text))
        }
    }
}


/// Handle GET for /proxies/list - list all proxies
#[get("/proxies/list")]
pub fn web_proxies_get_handler() -> Json<Proxies> {
    Json(Proxies::default())
}
