use std::time::Instant;

fn main() {
    let mut t0 = Instant::now();
    let (mut initial_state, moves) = load_input();
    println!("Initial parsing took {:?}", t0.elapsed());
    t0 = Instant::now(); 
    let r = v1(&mut initial_state, &moves);
    println!("Solution ({}) found in {:?}",r, t0.elapsed());
}



fn v1<'life>(initial_state : &'life mut Vec<Vec<char>>, moves : &'life Vec<(u8,usize,usize)>) -> String {
    for (amount, from, to) in moves {
        let mut from_column = initial_state[from-1].clone();
        let mut to_column = initial_state[to-1].clone();
        for _ in 0..(*amount as u8) {
            to_column.push(from_column.pop().unwrap());
        }
        initial_state[from-1] = from_column;
        initial_state[to-1] = to_column;
    }
    let result_vec = initial_state.iter().map(
            |entry|
            entry.clone().pop().unwrap()
        );
    let vec = result_vec.clone().collect::<Vec<_>>();
    let res_val = vec.iter().map(|c| c.to_string()).reduce(|a,b| a.as_str().to_owned()+b.as_str()).unwrap();
    res_val
}




fn load_input() -> (Vec<Vec<char>>, Vec<(u8,usize,usize)>) {
    let data = include_str!("../input.txt");
    let lines = data.lines();
    let mut initial_state = lines.clone()
        .take_while(|line| !line.contains("move"))
        .map(|line| line.replace("    ", "  ").replace("[","").replace("]", ""))
        .collect::<Vec<_>>()
    ;
    initial_state.truncate(initial_state.len()-2);
    let mut state_0 : Vec<Vec<char>> = vec![];
    for state in initial_state.iter().rev() {
        for (index, letter) in state.chars().step_by(2).enumerate() {
            while state_0.len() <= index {
                let column : Vec<char> = vec![];
                state_0.push(column);
            }
            if letter != ' ' {
                state_0[index].push(letter);
            }
        }
    }

    let moves = lines.skip_while(|line| !line.contains("move")).map(|line| line.replace("move ", "").replace(" from ", " ").replace(" to ", " "));
    let coded = moves.map(
        |line|
        {
            let (a,rest) = line.split_once(" ").unwrap();
            let (b,c) = rest.split_once(" ").unwrap();
            (
            a.parse::<u8>().unwrap(),
            b.parse::<usize>().unwrap(),
            c.parse::<usize>().unwrap(),
            )
        }
    ).collect::<Vec<(u8,usize,usize)>>();
    (state_0, coded)
}
