use crate::utils::folder_searcher::folderSearcher::getAllFilesInsideFolder;
use rand;
fn getDelimiter() -> char {
    return '#';
}
pub struct Layer {
    pub name: String,
    folder_path: String,
    pub image_paths: Vec<String>,
    pub num_traits: u32,
}

impl Layer {
    pub fn new(name: String, folder_path: String) -> Self {
        let image_paths = getAllFilesInsideFolder(&folder_path);
        let mut image_paths_with_rarity: Vec<String> = Vec::new();
        let mut index_in_backgrounds: u32 = 0;
        for i in image_paths.iter() {
            if i.contains(getDelimiter()) {
                let split_fn: Vec<&str> = i.split("#").collect();
                // println!("split fn = {:?}",&split_fn);
                let temp: Vec<&str> = split_fn[1].split(".").collect();
                let num_times_to_append: u32 = temp[0].parse().unwrap();

                for j in 0..num_times_to_append {
                    image_paths_with_rarity.push(i.clone());
                }
            } else {
                image_paths_with_rarity.push(i.clone());
            }
        }
        Self {
            name,
            folder_path,
            image_paths: image_paths_with_rarity,
            num_traits: image_paths.len() as u32,
        }
    }

    pub fn get_random_image_path(&self) -> String {
        let random_index = rand::random::<usize>() % self.image_paths.len();
        self.image_paths[random_index].clone()
    }
}
