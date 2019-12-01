use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::ops::Add;

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);

    let mut module_sum: i32 = 0;
    let mut modules: Vec<i32> = Vec::new();

    for line in f.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        modules.push(mass);
    }

    for mass in modules {
        module_sum += module2fuel(mass);
    }
    println!("Sum: {}", module_sum);

    Ok(())
}

fn module2fuel(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 { fuel + module2fuel(fuel) } else { 0 }
}