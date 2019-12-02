use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);


    let line = f.lines().nth(0).expect("Line read fault").expect("no fist line");
    let ops: Vec<&str> = line.split(',').collect();
    let mut ops: Vec<usize> = ops.iter().map(|code| code.parse::<usize>().unwrap()).collect();
  //  let mut ops: Vec<usize> = vec![1,1,1,4,99,5,6,0,99];//test data

    println!("Read in has {} codes", ops.len());

    ops[1]=12;
    ops[2]=2;
    let opcount = ops.len();
    for i in (0..opcount).step_by(4) {
        let opcode = ops[i];
        match opcode {
            1 => {
                println!("Opcode 1");
                let target = ops[i + 3];
                let p1 = ops[i + 1];
                let p2 = ops[i + 2];
                ops[target] = ops[p1] + ops[p2];
            }
            2 => {
                println!("Opcode 2");
                let target = ops[i + 3];
                let p1 = ops[i + 1];
                let p2 = ops[i + 2];
                ops[target] = ops[p1] * ops[p2];
            }
            99 => {
                println!("Dead");
                break;
            }
            _ => println!("Shit hit the fan."),
        }

    }

    println!("Result is: {}", ops.get(0).unwrap());

    Ok(())
}

