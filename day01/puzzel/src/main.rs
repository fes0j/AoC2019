use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}