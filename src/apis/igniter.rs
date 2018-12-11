/// Igniter Actions
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Actions {

    /// Create service
    Create,

    /// Destroy service
    Destroy,

    /// Start service
    Start,

    /// Stop service
    Stop,

    /// Restart service
    Restart,

    /// Reload service
    Reload,

    /// Test service
    Test,

    /// Watch service
    Watch,
}


/// Service igniter representation:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Igniter {

    /// Name of the service
    service_name: Option<String>,

    /// Igniter action
    action: Option<Actions>,

}
