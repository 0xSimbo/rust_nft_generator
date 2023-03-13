mod folder_searcher;
mod image_gen;
mod my_gen;

mod layer;
mod generator;

//define a static stirng called description
static DESCRIPTION: &'static str = "This is our super cool collection";
static START_TOKEN_ID: u32 = 0;
static END_TOKEN_ID: u32 = 10000;

fn main() {
    let layers = vec![
        layer::Layer::new(String::from("Backgrounds"),String::from("images/01_BACKGROUND")),
        layer::Layer::new(String::from("Body"),String::from("images/02_BODY")),
        layer::Layer::new(String::from("Clothing"),String::from("images/03_CLOTHING")),
        layer::Layer::new(String::from("Glasses"),String::from("images/04_GLASSES")),
        layer::Layer::new(String::from("Hats"),String::from("images/HATS")),
    ];

    let my_gen = generator::Generator::new(START_TOKEN_ID,END_TOKEN_ID,layers,DESCRIPTION);
    my_gen.run_generation();
}