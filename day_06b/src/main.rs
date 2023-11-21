use std::time::Instant;

fn main() {
    let data : &[u8; 4096] = include_bytes!("../input.txt");
    let t0 = Instant::now();
    let rvt = vt(&data);
    let dvt = t0.elapsed();
    println!("vt found {} in {:?}", rvt, dvt);
    let rvt = vext(&data);
    let dvt = t0.elapsed();
    println!("vext found {} in {:?}", rvt, dvt);
}

fn vt(data : &[u8; 4096]) -> u32 {
    const MEMORY: usize = 13;
    let mut history : [u32;MEMORY] = [0u32;MEMORY];
    let mut j = 0;
    for i in 0..4096 {
        let last_entry = history[(i+MEMORY-1)%MEMORY];
        let entry = 1u32 << (data[i] - 97);
        if last_entry ^ entry < last_entry {
            j = i;
            history = [0u32;MEMORY];
            history[i%MEMORY] = entry;
        } else {
        if j + MEMORY == i {
            return (j+MEMORY+1).try_into().unwrap();
        }
        history[i%MEMORY] = (last_entry & (!history[i%MEMORY])) | entry;
        }
    }
    return 0;
}

fn vext(data : &[u8;4096]) -> u32 {
    let mut i = 0;
    while let Some(win) = data.get(i..i + 14) {
        let mut seen = 0u32;
        if let Some(pos) = win.iter().rposition(|b| {
            let bit = 1 << (b % 32);
            let duplicate = seen & bit != 0;
            seen |= bit;
            duplicate
        }) {
            i += pos + 1;
        } else {
            break;
        }
    }
    i.try_into().unwrap()
}
