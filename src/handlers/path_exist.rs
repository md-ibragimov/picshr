use std::path::Path;

pub fn is_path_exist(path_input: &String) -> bool {
    let path = Path::new(path_input);

    if path.exists() & path.is_file() {
        return true;
    }
    
    return false;
}