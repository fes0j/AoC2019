use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::ops::Add;

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);

    let mut module_sum1: i32 = 0;
    let mut module_sum2: i32 = 0;
    let mut modules: Vec<i32> = Vec::new();

    for line in f.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        modules.push(mass);
    }

    for mass in &mut modules {
        let fuel = (*mass / 3) -2;
        module_sum1 = module_sum1.add( fuel);
    }
    println!("Our needed fuel is: {}", module_sum1);

    for mass in &mut modules {
        module_sum2 += module2fuel(*mass);
    }
    println!("Oh no! It really is  {}", module_sum2);

    Ok(())
}

fn module2fuel(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 { fuel + module2fuel(fuel) } else { 0 }
}