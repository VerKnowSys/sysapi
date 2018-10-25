#[derive(Debug, Serialize)]
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


#[derive(Debug, Serialize)]
pub struct Igniter {
    service_name: String,
    action: Actions,
}
