use glob::glob;


use api::SENTRY_PATH;


/// Produce list of dirs/files matching given glob pattern:
pub fn produce_list(glob_pattern: &String) -> Vec<String> {
    let mut list = vec!();
    for entry in glob(&glob_pattern).unwrap() {
        match entry {
            Ok(path) => {
                match path.file_name() {
                    Some(element) => {
                        element
                            .to_str()
                            .and_then(|elem| {
                                list.push(elem.to_string());
                                Some(elem.to_string())
                            });
                    },
                    None => (),
                }
            },
            Err(err) => {
                error!("Error: list_dirs(): {}", err);
            },
        }
    }
    debug!("produce_list(): Elements: {:?}", list);
    list
}


// Lists all cell attributes => /Shared/Prison/Sentry/CELLNAME/cell-attributes/*
pub fn list_attributes(cell_name: &String) -> Vec<String> {
    let glob_pattern = format!("{}/{}/cell-attributes/*", cell_name, SENTRY_PATH);
    produce_list(&glob_pattern)
}


/// Lists all available cells based on files found under Sentry dir:
pub fn list_cells() -> Vec<String> {
    let glob_pattern = format!("{}/*", SENTRY_PATH);
    produce_list(&glob_pattern)
}

