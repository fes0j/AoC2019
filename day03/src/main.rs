use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use array_tool::vec::Intersect;


#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);

    let curves = extract_curves(f);
    let curve1 = curves[0].clone();
    let curve2 = curves[1].clone();
    let mut intersections =  curve1.intersect(curve2);
    println!("There are {} intersections", intersections.len());
    intersections.sort_by(
        |a, b|
            a.m_distance().partial_cmp(&b.m_distance())
                .unwrap()
    );
    let int: Point = *intersections.first().unwrap();
    println!("Point: x={}, y={}, distance: {}. ",int.x, int.y, int.m_distance());
    Ok(())
}

fn extract_curves(f: BufReader<File>) -> Vec<Vec<Point>> {
    let mut curves: Vec<Vec<Point>> = vec![];
//Each line a line
    for path in f.lines() {
        let instructions = path.expect("line parse error");
        let instructions: Vec<&str> = instructions.split(',').collect();

        let mut curve: Vec<Point> = vec![];

        for instruction in instructions {
            let mut chars = instruction.chars();
            let direction = chars.next();
            let distance: i32 = chars.as_str().parse::<i32>().unwrap();


            let last_coordinate = match curve.last() {
                Some(p) => p.clone(),
                None => Point { x: 0, y: 0 },
            };
            match direction.unwrap() {
                'U' => {
                    for i in 1..distance {
                        curve.push(Point { x: last_coordinate.x, y: last_coordinate.y + i });
                    }
                }
                'D' => {
                    for i in 1..distance {
                        curve.push(Point { x: last_coordinate.x, y: last_coordinate.y - i });
                    }
                }
                'L' => {
                    for i in 1..distance {
                        curve.push(Point { x: last_coordinate.x - i, y: last_coordinate.y });
                    }
                }
                'R' => {
                    for i in 1..distance {
                        curve.push(Point { x: last_coordinate.x + i, y: last_coordinate.y });
                    }
                }
                _ => print!("kaputt"),
            }
        }
        println!("Curve size: {}", curve.len());

        curves.push(curve);
    }
    curves
}

trait Distance {
    /// Distance to center
   ///
   ///  ## Return the Manhatten Distance to (0,0)
    fn m_distance(&self) -> i32;
    /// Distance to center
   ///
   ///  ## Return the euclidean Distance to (0,0)
    fn euklid_distance(&self) -> f64;
}

impl Distance for Point {
    fn m_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn euklid_distance(&self) -> f64 {
        let t: f64 = ((self.x * self.x) + (self.y * self.y)) as f64;
        t.sqrt()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
       self.x == other.x && self.y == other.y
    }
}