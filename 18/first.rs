use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq)]
struct State {
    x: i32,
    y: i32,
    doors: [bool; 26],
}

impl State {
    pub fn new() -> State {
        return State {
            x : 0,
            y : 0,
            doors : [false; 26],
        };
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    
    let mut start : State = State::new();
    let mut maze : HashMap<(i32, i32), char> = HashMap::new();

    let mut col: i32 = 0;
    for raw_line in BufReader::new(f).lines() {
        for (row, c) in raw_line.unwrap().trim().chars().enumerate() {
            if c == '@' {
                start.x = row as i32;
                start.y = col;
            }
            maze.insert((row as i32, col), c);
        }
        col += 1;
    }

    let mut index : HashMap<State, i32> = HashMap::new();
    let mut reverse_index: Vec<State> = Vec::new();
    let mut cost : HashMap<i32, i32> = HashMap::new();
    let mut pending : VecDeque<i32> = VecDeque::new();

    let deltas : Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    index.insert(start.clone(), 0);
    reverse_index.push(start.clone());
    cost.insert(0, 0);
    pending.push_back(0);

    let mut total = std::i32::MAX;
    while !pending.is_empty() {
        let idx = pending.pop_front().unwrap();

        let curr = reverse_index.get(idx as usize).unwrap().clone();
        let new_cost = *(cost.get(&idx).unwrap()) + 1;

        for delta in deltas.iter() {
            let new_x = curr.x + delta.0;
            let new_y = curr.y + delta.1;

            let tile = *(maze.get(&(new_x, new_y)).unwrap());
            if tile == '#' {
                continue;
            }

            let mut new_state = curr.clone();
            if tile >= 'A' && tile <= 'Z' {
                let key = (tile as u8 - 'A' as u8) as usize;
                if !new_state.doors[key] {
                    continue;
                }
            }

            if tile >= 'a' && tile <= 'z' {
                let key = (tile as u8 - 'a' as u8) as usize;
                new_state.doors[key] = true;
                if !new_state.doors.iter().any(|x| !x) {
                    println!("{}", new_cost);
                    total = std::cmp::min(new_cost, total);
                    continue;
                }
            }

            new_state.x = new_x;
            new_state.y = new_y;

            if let Some(ex_idx) = index.get(&new_state) {
                if *cost.get(ex_idx).unwrap() <= new_cost {
                    continue;
                }
                panic!("bad");
            }

            let new_idx = reverse_index.len() as i32;
            reverse_index.push(new_state.clone());
            index.insert(new_state.clone(), new_idx);
            cost.insert(new_idx, new_cost);
            pending.push_back(new_idx);
        }
    }

    println!("{}", total);
}

