# A Blazingly Fast NFT Image Generator Written In Rust
## Author Twitter: @0xSimon

This repo takes advantage of rust's multithreading and low level capability
to provide a blazingly fast NFT Image and Metadata Generator

## Commands

To build, run  ```rust cargo build --release```
to execute, run ```rust cargo run --release```


## Instructions
1.  Go To src/main.rs

Inspect the main file.

```rust
fn main() {
    let layers = vec![
        Layer::new(String::from("Backgrounds"),String::from("images/01_BACKGROUND")),
        Layer::new(String::from("Body"),String::from("images/02_BODY")),
        Layer::new(String::from("Clothing"),String::from("images/03_CLOTHING")),
        Layer::new(String::from("Glasses"),String::from("images/04_GLASSES")),
        Layer::new(String::from("Hats"),String::from("images/HATS")),
    ];

    let my_gen = Generator::new(START_TOKEN_ID,END_TOKEN_ID,layers,DESCRIPTION);
    my_gen.run_generation();
}
```

Each layer item takes in the name of the layer and the path to where its images lie.

You define the layer in which you want it to be pasted on the image.
 
i.e. Backgrounds first, body second, etc. etc...

You then create a ```Generator``` and define a start token , end token, the layers, and a description.
```rust    let my_gen = Generator::new(START_TOKEN_ID,END_TOKEN_ID,layers,DESCRIPTION);
```

The generation will start with start token and end with end token.

Then, the execution of generation runs through the function ```rust my_gen.run_generation()```