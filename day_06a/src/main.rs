use std::time::Instant;

fn main() {
    let data : &[u8; 4096] = include_bytes!("../input.txt");
    let t0 = Instant::now();
    let rvt = vt(&data);
    let dvt = t0.elapsed();
    println!("vt found {} in {:?}", rvt, dvt);
    let t0 = Instant::now();
    let rvt = vext(&data);
    let dvt = t0.elapsed();
    println!("vt ext found {} in {:?}", rvt, dvt);
}

fn vt(data : &[u8; 4096]) -> u32 {
    for i in 3..4096 {
        let byte0 = data[i];
        let byte1 = data[i-1];
        let byte2 = data[i-2];
        let byte3 = data[i-3];
        if byte0 != byte1 && byte0 != byte2 && byte0 != byte3 && byte1 != byte2 && byte1 != byte3 && byte2 != byte3 {
            return (i+1).try_into().unwrap();
        }

    }
    return 0;
}

fn vext(data : &[u8;4096]) -> usize{
       data 
            .windows(4)
            .position(|b| !(0..3).any(|i| (i + 1..4).any(|j| b[i] == b[j])))
            .unwrap()
            + 4
}

