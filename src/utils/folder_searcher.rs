pub mod folderSearcher {
    use std::fs;
    pub fn getAllFilesInsideFolder(folderPath: &str) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        let paths = fs::read_dir(folderPath).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let pathString = path.to_str().unwrap().to_string();
            if path.is_dir() {
                let mut subFiles = getAllFilesInsideFolder(&pathString);
                files.append(&mut subFiles);
            } else {
                files.push(pathString);
            }
        }
        files
    }
}
