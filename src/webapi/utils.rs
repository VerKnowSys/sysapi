use glob::glob;

use api::CELLS_PATH;


pub fn list_cells() -> Vec<String> {
    let pattern = format!("{}/**", CELLS_PATH);
    let mut list: Vec<String> = vec!();
    for entry in glob(&pattern).unwrap() {
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
    warn!("list_cells(): Elements: {:?}", list);
    list
}
