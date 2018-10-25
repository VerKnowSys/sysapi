use std::process::Command;
use std::io::{Error, ErrorKind};

// Load all internal modules:
use api::*;
use api::igniter::*;


#[derive(Debug, Serialize)]
pub struct Cell {
    pub name: String,
    pub ipv4: String,
    pub domain: String,
    pub action: Actions,
}


// impl IntoResponse for Cell {
//     fn into_response(self, state: &State) -> Response<Body> {
//         create_response(
//             state,
//             StatusCode::OK,
//             mime::APPLICATION_JSON,
//             serde_json::to_string(&self).expect("serialized product"),
//         )
//     }
// }


pub fn add_ssh_pubkey_to_cell(name: &String, ssh_pubkey: &String) -> Result<(), Error> {
    Command::new(GVR_BIN)
        .arg("set")
        .arg(name.clone())
        .arg(format!("key='{}'", ssh_pubkey))
        .output()
        .and_then(|add_ssh_pubkey| {
            if add_ssh_pubkey.status.success() {
                info!("add_ssh_pubkey:\n{}", String::from_utf8_lossy(&add_ssh_pubkey.stdout));
                Ok(())
            } else {
                let error_msg = format!("Something went wrong and key: '{}' couldn't be set for cell: {}. Please contact administator or file a bug!", ssh_pubkey, name);
                error!("{}", error_msg);
                Err(Error::new(ErrorKind::Other, error_msg))
            }
        })
}


pub fn destroy_cell(name: &String) -> Result<(), Error> {
    Command::new(GVR_BIN)
        .arg("destroy")
        .arg(name)
        .arg("I_KNOW_EXACTLY_WHAT_I_AM_DOING") // NOTE: special GVR_BIN argument - "non interactive destroy"
        .output()
        .and_then(|gvr_handle| {
            if gvr_handle.status.success() {
                debug!("destroy_cell():\n{}{}",
                       String::from_utf8_lossy(&gvr_handle.stdout),
                       String::from_utf8_lossy(&gvr_handle.stderr));
                info!("Cell destroyed: {}", name);
                Command::new(JAIL_BIN)
                    .arg("-r") // NOTE: Sometimes jail services are locking "some" resources for a very long time,
                    .arg(name) //       and will remain "started" until the-process-lock is released..
                    .output()  //       Let's make sure there's no running jail with our name after destroy command:
                    .and_then(|jail_handle| {
                        if jail_handle.status.success() {
                            warn!("Dangling cell stopped: {}!", name);
                        }
                        Ok(())
                    })
                    // .map_err(|err| {
                    //     debug!("No dangling cell found. Looks clean.");
                    //     err
                    // })
            } else {
                Err(Error::new(ErrorKind::Other, format!("Couldn't destroy_cell(): {}", name)))
            }
        })
        .map_err(|err| {
            let error_msg = format!("Failure: {}", err);
            error!("ERROR: {}", error_msg);
            debug!("DEBUG(err): {:?}", err);
            err
        })
}

