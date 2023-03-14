# A Blazingly Fast NFT Image Generator Written In Rust
## Author Twitter: @0xSimon

This repo takes advantage of rust's multithreading and low level capability
to provide a blazingly fast NFT Image and Metadata Generator

Big thanks to hashlips for making his repo open source. I used his demo images
in this repo, so all credits to him.

Check out his repo <a href="https://github.com/HashLips/hashlips_art_engine">here</a>

## Benchmark Against Hashlips Generating 100 Images :: About 14x faster

### Hashlips: 9.71 seconds
### This Repo:  .7 seconds

<em>Note: this will be affected by your computer's specs since we multithread based on the number of cores on the computer.</em>

## Commands

To install rust, visit <a href="https://www.rust-lang.org/tools/install">The Official Rust lang Install Tutorial </a>

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

## Rarity

Rarity works the same as in the hashlips repo.

Each image should have "#x" after the filename. The x is the weight of the image in the overall generation.

For example, let's say we have 3 images in a certain folder.

1.  blue_hat#5.png
2.  red_hat#4.png
3.  special_hat#1.png

The probability of blue_hat would be (5+4+1) /5 = 50% Chance.
The probability of red_hat would be (5+4+1) / 4 = 40% Chance
the probablity of special_hat would be (5+4+1)/1 = 10% Chance.


Side Note: No Duplicates Can Be Generated From This Script