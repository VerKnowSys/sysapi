#[derive(Debug, Serialize, Deserialize)]
pub enum Actions {
    Create,
    Destroy,
    Start,
    Stop,
    Restart,
    Reload,
    Test,
    Watch,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Igniter {
    service_name: String,
    action: Actions,
}
