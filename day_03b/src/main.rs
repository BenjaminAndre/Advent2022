use std::time::Instant;

fn main() {
    let data = include_str!("../input.txt");
    let t0 = Instant::now();
    let r = v1(&data);
    let t1 = t0.elapsed();
    println!("Solution v1 ({}) found in {:?}", r, t1);
}

fn v1(data : &str) -> u32 {
    data
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(
        |set| 
        pocketsToFlagArray(set)
        )
        .map(|repr| repr.ilog2())
        .sum::<u32>()
}


fn pocketsToFlagArray(bag : &[&str]) -> u64 {
    let pocket1Value = bag[0].as_bytes().iter().map(|b| asciiToFlagArray(b)).reduce(|c,d| c | d).unwrap();
    let pocket2Value = bag[1].as_bytes().iter().map(|b| asciiToFlagArray(b)).reduce(|c,d| c | d).unwrap();
    let pocket3Value = bag[2].as_bytes().iter().map(|b| asciiToFlagArray(b)).reduce(|c,d| c | d).unwrap();
    pocket1Value & pocket2Value & pocket3Value
}

fn asciiToFlagArray(ascii : &u8) -> u64 {
    if ascii >= &97u8 {
        0b1u64 << (ascii - 96u8)
    }
    else {
        0b1u64 << (ascii - 38u8) 
    } 
}
