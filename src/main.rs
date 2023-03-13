mod utils {
    pub mod folder_searcher;
    pub mod my_gen;
    pub mod image_gen;
    pub mod layer;
    pub mod generator;
}
// mod image_gen;
// mod my_gen;
use utils::generator::{Generator};
use utils::layer::{Layer};
// mod layer;
// mod generator;

//define a static stirng called description
static DESCRIPTION: &'static str = "This is our super cool collection";
static START_TOKEN_ID: u32 = 0;
static END_TOKEN_ID: u32 = 10000;

fn main() {
    let layers = vec![
        Layer::new(String::from("Backgrounds"),String::from("layers/01_BACKGROUND")),
        Layer::new(String::from("Body"),String::from("layers/02_BODY")),
        Layer::new(String::from("Clothing"),String::from("layers/03_CLOTHING")),
        Layer::new(String::from("Glasses"),String::from("layers/04_GLASSES")),
        Layer::new(String::from("Hats"),String::from("layers/HATS")),
    ];

    let my_gen = Generator::new(START_TOKEN_ID,END_TOKEN_ID,layers,DESCRIPTION);
    my_gen.run_generation();
}