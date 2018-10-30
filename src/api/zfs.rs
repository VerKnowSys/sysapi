#[derive(Debug, Serialize)]
pub struct Rollback {
    name: String,
    dataset_path: String,
}


#[derive(Debug, Serialize)]
pub struct Snapshot {
    name: String,
    dataset_path: String,
}
