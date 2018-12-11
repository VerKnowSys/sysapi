use gotham::state::State;
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use gotham::helpers::http::response::create_response;
use mime::*;


/// Owner structure:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Owner {

    /// Owner UserID:
    pub uid: Option<String>,

    /// Owner SSH pubkey:
    pub key: Option<String>,

    /// Owner state:
    pub state: OwnerState,
}


/// Owners (Owner List) structure for easy list management
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Owners {

    /// List of all owners
    pub list: Vec<Owner>

}



/// State of the Owner
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum OwnerState {

    /// Owner is Normal:
    Normal,

    /// Owner is Locked:
    Locked,

    /// Owner doesn't exist:
    NotFound,
}


/// Serialize to JSON on .to_string()
impl ToString for Owner {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


/// Serialize to JSON on .to_string()
impl ToString for Owners {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


/// Implement response for GETs:
impl IntoResponse for Owner {
    fn into_response(self, state: &State) -> Response<Body> {
        // serialize only if uid is set - so Cell is initialized/ exists
        match self.uid {
            Some(_) =>
                create_response(
                    state,
                    StatusCode::OK,
                    APPLICATION_JSON,
                    serde_json::to_string(&self)
                        .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}")),
                ),
            None =>
                create_response(
                    state,
                    StatusCode::NOT_FOUND,
                    APPLICATION_JSON,
                    Body::from("{\"status\": \"NotFound\"}"),
                )
        }
    }
}


/// Implement response for GETs:
impl IntoResponse for Owners {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            APPLICATION_JSON,
            serde_json::to_string(&self)
                .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}")),
        )
    }
}


impl Owner {


    /// Create new Owner
    pub fn new(name: &String, key: &String) -> Owner {
        Owner {
            name: Some(name.to_string()),
            key: Some(key.to_string()),
            state: OwnerState::Normal,
        }
    }


}
