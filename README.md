# A Blazingly Fast NFT Image Generator Written In Rust
## Author Twitter: @0xSimon

### Disclaimer: Only PNG Files Supported. Will add support for other image types later
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
    utils::before_runtime::before_runtime();
    let start_time = std::time::Instant::now();
    let layers = vec![
        Layer::new(String::from("Backgrounds"),String::from("layers/Background")),
        Layer::new(String::from("Bottom Lid"),String::from("layers/Bottom lid")),
        Layer::new(String::from("Eye Color"),String::from("layers/Eye color")),
        Layer::new(String::from("Eyeball"),String::from("layers/Eyeball")),
        Layer::new(String::from("Goo"),String::from("layers/Goo")),
        Layer::new(String::from("Iris"),String::from("layers/Iris")),
        Layer::new(String::from("Shine"),String::from("layers/Shine")),
        Layer::new(String::from("Top Lid"),String::from("layers/Top lid")),


    ];

    let my_gen = Generator::new(START_TOKEN_ID,END_TOKEN_ID,layers,DESCRIPTION);
    my_gen.run_generation();

    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Time taken to generate {} images: {:?}",END_TOKEN_ID,duration);
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