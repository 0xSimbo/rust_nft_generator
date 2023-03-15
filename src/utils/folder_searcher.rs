pub mod folder_searcher {
    use std::fs;
    pub fn get_all_files_inside_folder(folderPath: &str) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        let paths = fs::read_dir(folderPath).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let path_string = path.to_str().unwrap().to_string();
            if path.is_dir() {
                let mut subFiles = get_all_files_inside_folder(&path_string);
                files.append(&mut subFiles);
            } else {
                files.push(path_string);
            }
        }
        files
    }
}
