use std::time::{Instant};

fn main(){
    let data = include_str!("../data.txt");
    let t0 = Instant::now();
    let result = vext(data);
    println!("Time spent on Vext ({}): {:?}", result, t0.elapsed());
    let t0 = Instant::now();
    let result = v1(data);
    println!("Time spent on V1 ({}): {:?}", result, t0.elapsed());
}




fn vext(data : &str) -> u32 {
    let mut cals = data
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    cals.sort_unstable();
    cals.into_iter().rev().take(3).sum::<u32>()
}


fn v1(data : &str) -> u32 {
    let mut vec = data
        .split("\n\n")
        .map(|elf| elf
            .lines()
            .map(|value| value.parse::<u32>().unwrap())
            .sum()
        )
        .collect::<Vec<u32>>();
    vec.sort_by(|a,b| b.cmp(a));
    vec.into_iter().take(3).sum::<u32>()
}
