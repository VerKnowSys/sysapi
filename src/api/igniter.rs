#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Igniter {
    service_name: Option<String>,
    action: Option<Actions>,
}
