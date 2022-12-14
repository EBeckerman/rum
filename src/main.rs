mod read_bin;
pub mod instruct;
pub mod bitpack;
use std::env;

// use std::time::Instant;

// use crate::instruct;

fn main() {
    // let now = Instant::now();

    let input = env::args().nth(1);
    let instructions: Vec<u32> = read_bin::load(input.as_deref());
    read_bin::instructs(instructions.clone());

    // let elapsed = now.elapsed();

    // println!("{:.2?}", elapsed);
    // println!("{}", instructions.len());
}

/*
Read Binary File
Stitch 4 bits together to a u32
Read the instructions in the u32
 - First 4 bits: Op Code
 - Next 19 bits: Instructions
 - Last 9 bits:
    - Register A
    - Register B
    - Register C
*/