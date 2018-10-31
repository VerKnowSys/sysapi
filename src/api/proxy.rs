#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proxy {

    /// Proxy from URL:
    from: Option<String>,

    /// Proxy to URL:
    to: Option<String>,

}
