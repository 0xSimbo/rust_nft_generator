use std::io::Write;

use crate::utils::layer::Layer;
use crate::utils::image_gen::{generate};
use serde_json::{json};
use serde_json;
use std::path::Path;

use serde::{Serialize, Deserialize};

use num_cpus;
 pub struct Generator {
    start_token_id: u32,
    end_token_id: u32,
    layers: Vec<Layer>,
    //description is a reference to a static string
    description:  &'static str,
}
#[derive(Serialize, Deserialize)]
struct Attribute {
    trait_type: String,
    value: String,
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
    pub fn run_generation(&self) {
        let num_cpus = num_cpus::get() as u32;
        let mut threads: Vec<std::thread::JoinHandle<()>> = Vec::new();
        let description = self.description;
        let num_cycles = (self.end_token_id - self.start_token_id) / num_cpus;
        // let mut start = self.start_token_id;
        // let mut end = start + num_cycles;
        let start_token = self.start_token_id;

        for  i in 0..num_cycles {

            let mut threads:Vec<std::thread::JoinHandle<()>> = Vec::new();
            
            for j in 0..num_cpus {
          
               let random_image_names = self.layers.iter().map(|layer| {
                    layer.get_random_image_path()
                }).collect::<Vec<String>>();

                let mut attributes:Vec<Attribute> = Vec::new();
                for k in 0..self.layers.len() {
                    //To find the name of the trait we need to split it on # and take the first element amd then split it on . and take the first element again 
                    let file_path = Path::new(&random_image_names[k]);
                    let file_name = String::from(file_path.file_name().unwrap().to_str().unwrap());
                    if file_name.contains("#")  {
                        let split_fn:Vec<&str> = file_name.split("#").collect();
                        let temp:Vec<&str> = split_fn[1].split(".").collect();
                        let num_times_to_append:u32 = temp[0].parse().unwrap();
                        let attribute = Attribute {
                            trait_type: self.layers[k].name.clone(),
                            value: format!("{}#{}",split_fn[0],num_times_to_append),
                        };
                        attributes.push(attribute);
                        
                    }
                    else{
                        let split_fn:Vec<&str> = file_name.split(".").collect();
                        let attribute = Attribute {
                            trait_type: self.layers[k].name.clone(),
                            value: split_fn[0].to_string(),
                        };
                        attributes.push(attribute);
                    }
                    //if value contains # split it on # and take the first element else split it on . and take the first element
                    

                }
                let curr_id = (&num_cpus * &i)+ &j + &start_token;
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
                    // let serialized = serde_json::to_string_pretty(&json_file).unwrap();
                    // std::fs::write(format!("./build/json/{}.json",&curr_id),serialized).unwrap();
                });
                threads.push(thread);

                



            };
            //Save the json

            for thread in threads {
                thread.join().unwrap();
            }

      
        }



        // for i in 0..num_cpus {
        //     let thread = std::thread::spawn(move || {
        //         let random_image_names = self.layers.iter().map(|layer| {
        //             layer.get_random_image_path()
        //         }).collect::<Vec<String>>();
        //         generate(format!("./build/images/{}.png",i).as_str(),random_image_names);
        //     });
        //     threads.push(thread);
        // }

        for thread in threads {
            thread.join().unwrap();
        }
        // for i in self.start_token_id..self.end_token_id {
        //     let random_image_names = self.layers.iter().map(|layer| {
        //         layer.get_random_image_path()
        //     }).collect::<Vec<String>>();
        //     generate(format!("./build/images/{}.png",i).as_str(),random_image_names);
        // }
    }
}