use std::time::Instant;

fn main() {
    let data = include_str!("../input.txt");
    let t0 = Instant::now();
    let rvt = vt(data);
    let dvt = t0.elapsed();
    println!("VT found {} in {:?}", rvt, dvt);
}

fn vt(data : &str) -> usize {
    data.lines()
        .map(|line|
            {
            let (l,r) = line.split_once(",").unwrap();
            let ((ll,lh),(rl,rh)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
            (
                ll.parse::<u8>().unwrap(),
                lh.parse::<u8>().unwrap(),
                rl.parse::<u8>().unwrap(),
                rh.parse::<u8>().unwrap(),
            )
            }
        )
        .filter(|(ll,lh,rl,rh)|
            (ll >= rl && lh <= rh) ||
            (rl >= ll && rh <= lh)
            )
        .count()
}

