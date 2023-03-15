use crate::utils::folder_searcher::folderSearcher::getAllFilesInsideFolder;

fn getDelimiter() -> char {
    return '#';
}
pub fn getAllBackgrounds() -> Vec<String> {
    let backgrounds = getAllFilesInsideFolder("images/01_BACKGROUND");
    let mut bg_with_rarity: Vec<String> = Vec::new();
    for i in backgrounds.iter() {
        if i.contains(getDelimiter()) {
            let split_fn: Vec<&str> = i.split("#").collect();
            println!("split fn = {:?}", &split_fn);
            let temp: Vec<&str> = split_fn[1].split(".").collect();
            let num_times_to_append: u32 = temp[0].parse().unwrap();

            for j in 0..num_times_to_append {
                bg_with_rarity.push(i.clone());
            }
        } else {
            bg_with_rarity.push(i.clone());
        }
    }
    return bg_with_rarity;
}

pub fn getAllBodies() -> Vec<String> {
    let bodies = getAllFilesInsideFolder("images/02_BODY");
    return bodies;
}

pub fn getAllClothing() -> Vec<String> {
    let clothing = getAllFilesInsideFolder("images/03_CLOTHING");
    return clothing;
}

pub fn getAllGlasses() -> Vec<String> {
    let glasses = getAllFilesInsideFolder("images/04_GLASSES");
    return glasses;
}
pub fn getAllHairs() -> Vec<String> {
    let hairs = getAllFilesInsideFolder("images/HATS");
    return hairs;
}

//    pub fn
