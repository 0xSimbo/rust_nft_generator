use std::io::Write;

use crate::utils::layer::Layer;
use crate::utils::attribute::Attribute;
use crate::utils::attribute::{hash_attributes};

// use crate::utils::attribute::Attribute;
use crate::utils::image_gen::{generate};

use serde_json::{json};
use serde_json;
use std::path::Path;


use serde::{Serialize, Deserialize};
struct ImageData {
    random_image_names: Vec<String>,
    attributes: Vec<Attribute>,
}
use num_cpus;
 pub struct Generator {
    start_token_id: u32,
    end_token_id: u32,
    layers: Vec<Layer>,
    //description is a reference to a static string
    description:  &'static str,
}

impl Generator {
    pub fn new (start_token_id: u32, end_token_id: u32, layers: Vec<Layer>,description:&'static str) -> Self {
        Self {
            start_token_id,
            end_token_id,
            layers,
            description: description,
        }
    }

    // fn get_single_nft_data(&self) -> Vec<String> {

    // }
    fn generate_all_images_metadata_and_check_duplicates(&self) -> Vec<ImageData> {
        let num_images_per_layer_as_vec = self.layers.iter().map(|layer| {
            layer.num_traits
        }).collect::<Vec<u32>>();

        //FIND THE NUMBER OF POSSIBLE COMBINATIONS AS A U128
        let mut num_possible_combinations:u128 = 1;
        for num_images_per_layer in num_images_per_layer_as_vec.iter() {
            num_possible_combinations = num_possible_combinations * (*num_images_per_layer as u128);
        }
        let total_images_to_generate = self.end_token_id - self.start_token_id;
        if total_images_to_generate as u128 > num_possible_combinations {
            panic!("The number of images to generate is greater than the number of possible combinations");
        }


        let mut metadata_hashes = Vec::new();
        let mut image_data:Vec<ImageData> = Vec::new();
        let mut i = self.start_token_id;
        //loop from start id to end id
        while i < self.end_token_id {
            let mut random_image_names = self.layers.iter().map(|layer| {
                layer.get_random_image_path()
            }).collect::<Vec<String>>();

            let mut attributes:Vec<Attribute> = Vec::new();
            for j in 0..self.layers.len() {
                //To find the name of the trait we need to split it on # and take the first element amd then split it on . and take the first element again 
                let file_path = Path::new(&random_image_names[j]);
                let file_name = String::from(file_path.file_name().unwrap().to_str().unwrap());
                let trait_type = file_name.split("#").collect::<Vec<&str>>()[0].split(".").collect::<Vec<&str>>()[0];
                let value = file_name.split("#").collect::<Vec<&str>>()[1].split(".").collect::<Vec<&str>>()[0];
                attributes.push(Attribute::new(String::from(trait_type),String::from(value)));
            }


            let metadata_hash = hash_attributes(&attributes);
            if metadata_hashes.contains(&metadata_hash) {
                println!("Duplicate metadata found, regenerating");
                // i = i-1;
                // continue;
            } else {
                // let image_name = format!("{}.png",i);
                // let stringified_json = serde_json::to_string(&json!({
                //     "name": format!("NFT #{}",i),
                //     "description": self.description,
                //     "image": format!("ipfs://{}",image_name),
                //     "attributes": &attributes
                // })).unwrap();

                image_data.push(ImageData {
                    // image_name,
                    random_image_names,
                    attributes,
                    // stringified_json,
                });
                metadata_hashes.push(metadata_hash.clone());
                i = i + 1;
            }

        }



   
        return  image_data;



    }
    pub fn run_generation(&self) {
        let num_cpus = num_cpus::get() as u32;
        let mut threads: Vec<std::thread::JoinHandle<()>> = Vec::new();
        let description = self.description;
        let num_cycles = (self.end_token_id - self.start_token_id) / num_cpus;
        // let mut start = self.start_token_id;
        // let mut end = start + num_cycles;
        let start_token = self.start_token_id;

        let image_data = self.generate_all_images_metadata_and_check_duplicates();

        for  i in 0..num_cycles {

            let mut threads:Vec<std::thread::JoinHandle<()>> = Vec::new();
            
            for j in 0..num_cpus {
                
                let curr_id = (&num_cpus * &i)+ &j + &start_token;
                let random_image_names = image_data[curr_id as usize].random_image_names.clone();
                let attributes = image_data[curr_id as usize].attributes.clone();
            

      
                let thread = std::thread::spawn(move || {
                    generate(format!("./build/images/{}.png",&curr_id).as_str(),random_image_names);
                
                
                    let json_file = json!({
                        "name": format!("#{}",&curr_id),
                        "description": description,
                        "image": format!("ipfs://ipfsHash/{}.png",&curr_id),
                        "attributes": serde_json::to_string(&attributes).unwrap(),
                    });

                    let serialized = serde_json::to_string_pretty(&json_file).unwrap();
                    let mut file = std::fs::File::create(format!("./build/json/{}.json",curr_id)).unwrap();
                    file.write(serialized.as_bytes()).unwrap();

            
                });
                threads.push(thread);

                



            };

            
            //Save the json

            for thread in threads {
                thread.join().unwrap();
            }

      
        }

        for thread in threads {
            thread.join().unwrap();
        }

    }
}