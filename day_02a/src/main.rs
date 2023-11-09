#![feature(iter_array_chunks)]

use std::time::{Instant};

fn main(){
    let data = include_str!("../input.txt");
    let t0 = Instant::now();
    let result = v1(data);
    println!("Time spent on V1 ({}): {:?}", result, t0.elapsed());
    // vext();
    v2(data);
}

fn v1(data :&str) -> u32 {
    data.lines().map(|line| result(line)).sum()
}

fn result(line : &str) -> u32 {
  match line {
    "A X" => 4,
    "A Y" => 8,
    "A Z" => 3,
    "B X" => 1,
    "B Y" => 5,
    "B Z" => 9,
    "C X" => 7,
    "C Y" => 2,
    "C Z" => 6,
    _ => panic!("Data badly parsed"),
    }
}

fn vext() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
            .sum::<i16>(),
    );
}

fn v2(data :&str) {
    let rows = data.trim().as_bytes().array_chunks::<4>().map(|[p1, _, p2, _]| p1 + p2);
    println!("{:?}", rows);
}
