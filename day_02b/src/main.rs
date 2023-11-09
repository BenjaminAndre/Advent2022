use std::time::{Instant};

fn main(){
    let data = include_str!("../input.txt");
    let t0 = Instant::now();
    let result = v1(data);
    println!("Time spent on V1 ({}): {:?}", result, t0.elapsed());
}

fn v1(data :&str) -> u32 {
    data.lines().map(|line| result(line)).sum()
}

fn result(line : &str) -> u32 {
  match line {
    "A X" => 3,
    "A Y" => 4,
    "A Z" => 8,
    "B X" => 1,
    "B Y" => 5,
    "B Z" => 9,
    "C X" => 2,
    "C Y" => 6,
    "C Z" => 7,
    _ => panic!("Data badly parsed"),
    }
}
