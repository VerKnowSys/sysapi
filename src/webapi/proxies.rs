use hyper::*;
use futures::{future, Future, Stream};
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use mime::*;


// Load all internal modules:
use api::*;
use api::proxy::*;


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

                let proxy = Proxy::new(&from, &to)
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
