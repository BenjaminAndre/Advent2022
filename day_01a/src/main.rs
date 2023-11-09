use std::time::{Duration, Instant};

fn main(){
    let data = include_str!("../data.txt");
    let t0 = Instant::now();
    let result = v1(data);
    println!("Time spent on V1 ({}): {:?}", result, t0.elapsed());
    let t0 = Instant::now();
    let result = v2(data);
    println!("Time spent on V2 ({}): {:?}", result, t0.elapsed());
}




fn v1(data : &str) -> u32 {
    let calories = data.split("\n\n").map(|elf| elf.lines().map(|value| value.parse::<u32>().unwrap()).sum());
    let largest : u32 = calories.clone().max().unwrap();
    match calories.map(|e| e.to_string()).reduce(|a, b| a + "\n" + &b) {
        Some(expr) => println!("{expr}"),
        None => println!("Elves couldn't be parsed"),
    }
    largest
}


fn v2(data : &str) -> u32 {
    data.split("\n\n").map(|elf| elf.lines().map(|value| value.parse::<u32>().unwrap()).sum()).max().unwrap()
}
