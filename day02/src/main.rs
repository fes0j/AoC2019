use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);


    let line = f.lines().nth(0).expect("Line read fault").expect("no fist line");
    let ops: Vec<&str> = line.split(',').collect();
    let ops: Vec<usize> = ops.iter().map(|code| code.parse::<usize>().unwrap()).collect();
  //  let mut ops: Vec<usize> = vec![1,1,1,4,99,5,6,0,99];//test data

    println!("Read in has {} codes", ops.len());

    let result1 = part1(ops.clone(),12,2);
    println!("Part1 result is: {}", result1);
    let (verb, noun) = part2(ops).expect("Error in loop");
    let input = 100* noun + verb;
    println!("Part2: Please put {} into the computer", input);

    Ok(())
}

fn part2(ops : Vec<usize>)-> Result<(usize, usize),()>{
    let output =19690720;

    for noun  in 0..99 {
        for verb in 0..99 {
           if output == part1(ops.clone(),noun,verb){
               return Ok((verb, noun))

           }
        }
    }

    Err(())


}

fn part1(mut ops : Vec<usize>, input1 : usize, input2 : usize) -> usize {
    ops[1]=input1;
    ops[2]=input2;
    let opcount = ops.len();
    for i in (0..opcount).step_by(4) {
        let opcode = ops[i];
        match opcode {
            1 => {
                let target = ops[i + 3];
                let p1 = ops[i + 1];
                let p2 = ops[i + 2];
                ops[target] = ops[p1] + ops[p2];
            }
            2 => {
                let target = ops[i + 3];
                let p1 = ops[i + 1];
                let p2 = ops[i + 2];
                ops[target] = ops[p1] * ops[p2];
            }
            99 => {
                break;
            }
            _ => println!("Shit hit the fan."),
        }

    }
    let result =  ops.get(0).unwrap();
    *result
}
