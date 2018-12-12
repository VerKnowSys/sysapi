use hyper::*;
use futures::{future, Future, Stream};
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use mime::*;


use crate::*;
use crate::apis::proxy::*;


/// Handle POSTs /proxy/:cell/:from/:to
pub fn web_proxy_post_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(_valid_body) => {
                let uri = Uri::borrow_from(&state).to_string();
                let cell_from_to = uri.replace(PROXY_RESOURCE, "");
                let cell_name: String = cell_from_to.split("/").take(1).collect();
                let from: String = cell_from_to.split("/").skip(1).take(1).collect();
                let to: String = cell_from_to.split("/").skip(2).take(1).collect();

                // TODO: XXX: add some validations? Especially for malicious wildcard redirects or stuff like that :)
                // if from.len() < 4
                // || from.len() > 64

                let proxy = Proxy::create(&cell_name, &from, &to)
                    .and_then(|proxy_block| {
                        Ok(proxy_block.to_string())
                    });
                match proxy {
                    Ok(result_config) => {
                        let res_text = format!("{{\"status\": \"Successfully created new proxy configuration for cell: {}.\"}}", cell_name);
                        debug!("web_proxy_post_handler(): {}. RESULT-CONFIG: {}", res_text, result_config);
                        let res = create_response(&state, StatusCode::OK, APPLICATION_JSON, Body::from(res_text));
                        future::ok((state, res))
                    },
                    Err(err) => {
                        let res_text = format!("{{\"status\": \"Failed to create new proxy configuration for cell: {}. Error details: {}.\"}}", cell_name, err);
                        error!("{}", res_text);
                        let res = create_response(&state, StatusCode::BAD_REQUEST, APPLICATION_JSON, Body::from(res_text));
                        future::ok((state, res))
                    }
                }
            }
            Err(e) => future::err((state, e.into_handler_error()))
        });

    Box::new(f)
}


/// Handle DELETEs /proxy/:cell/:from/:to
pub fn web_proxy_delete_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(_valid_body) => {
                let uri = Uri::borrow_from(&state).to_string();
                let cell_from_to = uri.replace(PROXY_RESOURCE, "");
                let cell_name: String = cell_from_to.split("/").take(1).collect();
                let from: String = cell_from_to.split("/").skip(1).take(1).collect();
                let to: String = cell_from_to.split("/").skip(2).take(1).collect();
                debug!("web_proxy_delete_handler(): cell_name: {}, from: {}, to: {}", cell_name, from, to);

                match Proxy::destroy(&cell_name.to_string(), &from.to_string(), &to.to_string()) {
                    Ok(_) => {
                        let res_text = format!("{{\"status\": \"Successfully destroyed proxy configuration for cell: {}.\"}}", cell_name);
                        let res = create_response(&state, StatusCode::OK, APPLICATION_JSON, Body::from(res_text));
                        future::ok((state, res))
                    },
                    Err(err) => {
                        let res_text = format!("{{\"status\": \"Failed to destroy proxy configuration for cell: {}. Error details: {}.\"}}", cell_name, err);
                        error!("{}", res_text);
                        let res = create_response(&state, StatusCode::BAD_REQUEST, APPLICATION_JSON, Body::from(res_text));
                        future::ok((state, res))
                    }
                }
            }
            Err(e) => future::err((state, e.into_handler_error()))
        });

    Box::new(f)
}


/// Handle GET for /proxies/list - list all proxies
pub fn web_proxies_get_handler(state: State) -> (State, Proxies) {
    (state, Proxies::default())
}
