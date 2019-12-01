use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::ops::Add;

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);

    let mut module_sum : u32 = 0;
    for line in f.lines() {
         let mass = line.unwrap().parse::<u32>().unwrap();
        let fuel = (mass / 3) -2;
        module_sum = module_sum.add( fuel);
    }
    println!("Sum: {}", module_sum);

    Ok(())
}