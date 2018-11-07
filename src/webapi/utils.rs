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
                error!("Error: produce_list(): {}", err);
            },
        }
    }
    debug!("produce_list(): Elements: {:?}", list);
    list
}


/// Lists all cell attributes => /Shared/Prison/Sentry/CELLNAME/cell-attributes/*
pub fn list_attributes(cell_name: &String) -> Vec<String> {
    let glob_pattern = format!("{}/{}/cell-attributes/*", SENTRY_PATH, cell_name);
    debug!("list_attributes(): {}", glob_pattern);
    produce_list(&glob_pattern)
}


/// Lists all available cells based on files found under Sentry dir:
pub fn list_cells() -> Vec<String> {
    let glob_pattern = format!("{}/*", SENTRY_PATH);
    debug!("list_cells(): {}", glob_pattern);
    produce_list(&glob_pattern)
}


/// Lists all available proxies based on files found under Sentry dirs:
pub fn list_proxies() -> Vec<String> {
    let glob_pattern = format!("{}/**/cell-webconfs/*", SENTRY_PATH);
    debug!("list_proxies(): {}", glob_pattern);
    produce_list(&glob_pattern)
}
