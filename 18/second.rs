use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;


#[derive(Debug, Copy, Default, Clone, PartialEq, Eq)]
struct Edge {
    x : i32,
    y : i32,
    doors : [bool; 26],
    steps : i32,
}

#[derive(Debug, Hash, Copy, Default, Clone,  PartialEq, Eq)]
struct State {
    robots : [usize; 4],
    keys : [bool; 26],
}

fn main() {
    let f = File::open("input.txt").unwrap();
    
    let mut maze : HashMap<(i32, i32), char> = HashMap::new();

    let mut col: i32 = 0;
    let mut o_x : i32 = 0;
    let mut o_y : i32 = 0;
    let mut poi : HashMap<(i32, i32), char> = HashMap::new();

    for raw_line in BufReader::new(f).lines() {
        for (row, c) in raw_line.unwrap().trim().chars().enumerate() {
            if c == '@' {
                o_x = row as i32;
                o_y = col;
            }
            if c >= 'a' && c <= 'z' {
                poi.insert((row as i32, col), c);
            }
            maze.insert((row as i32, col), c);
        }
        col += 1;
    }

    maze.insert((o_x, o_y), '#'); 
    maze.insert((o_x-1, o_y), '#'); 
    maze.insert((o_x+1, o_y), '#'); 
    maze.insert((o_x, o_y+1), '#'); 
    maze.insert((o_x, o_y-1), '#');

    maze.insert((o_x-1, o_y-1), '@');
    maze.insert((o_x+1, o_y-1), '@');
    maze.insert((o_x-1, o_y+1), '@');
    maze.insert((o_x+1, o_y+1), '@');

    poi.insert((o_x-1, o_y-1), '{');
    poi.insert((o_x+1, o_y-1), '|');
    poi.insert((o_x-1, o_y+1), '}');
    poi.insert((o_x+1, o_y+1), '~');

    let mut graph : [[Edge; 26]; 30] = [<[Edge; 26]>::default(); 30];
    let deltas = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (xy, ch) in poi.iter() {
        let src = (*ch as u8 - 'a' as u8) as usize;

        let mut pending : VecDeque<Edge> = VecDeque::new();
        let mut visited : HashSet<(i32, i32)> = HashSet::new();

        let mut ori : Edge = Edge::default();
        ori.x = xy.0;
        ori.y = xy.1;
        pending.push_back(ori.clone());
        visited.insert(xy.clone());

        while !pending.is_empty() {
            let curr = pending.pop_front().unwrap().clone();
            for delta in deltas.iter() {
                let new_x = curr.x + delta.0;
                let new_y = curr.y + delta.1;
                if visited.contains(&(new_x, new_y)) {
                    continue;
                }

                let tile = *(maze.get(&(new_x, new_y)).unwrap());
                if tile == '#' {
                    continue;
                }

                let mut new_edge = curr.clone();
                new_edge.x = new_x;
                new_edge.y = new_y;
                new_edge.steps = curr.steps + 1;

                visited.insert((new_x, new_y));
                
                if tile >= 'a' && tile <= 'z' {
                    let index = (tile as u8 - 'a' as u8) as usize;
                    graph[src][index] = new_edge.clone();
                }
                
                if tile >= 'A' && tile <= 'Z' {
                    let door = (tile as u8 - 'A' as u8) as usize;
                    new_edge.doors[door] = true;
                }

                pending.push_back(new_edge.clone());
            }
        }
    }

    let mut ori = State::default();
    ori.robots[0] = 26;
    ori.robots[1] = 27;
    ori.robots[2] = 28;
    ori.robots[3] = 29;

    let mut pending: BTreeMap<i32, State> = BTreeMap::new();
    let mut costs: HashMap<State, i32> = HashMap::new();
    pending.insert(0, ori.clone());
    costs.insert(ori.clone(), 0);

    let mut total : i32 = std::i32::MAX;
    while !pending.is_empty() {
        let (est, curr_tmp) = pending.iter().next().unwrap();
        let curr = curr_tmp.clone();
        pending.remove(&est.clone());

        let ori_cost :i32 = *costs.get(&curr).unwrap();
        for i in 0..4 {
            for j in 0..26 {
                if curr.keys[j] {
                    continue;
                }

                if graph[curr.robots[i]][j].steps == 0 {
                    continue;
                }

                let mut bad = false;
                for k in 0..26 {
                    if graph[curr.robots[i]][j].doors[k] && !curr.keys[k] {
                        bad = true;
                        break;
                    }
                }
                if bad {
                    continue;
                }

                let new_cost : i32 = ori_cost + graph[curr.robots[i]][j].steps;
                let mut new_state = curr.clone();
                new_state.keys[j] = true;
                new_state.robots[i] = j;

                if let Some(exist_cost) = costs.get(&new_state) {
                    if *exist_cost <= new_cost {
                        continue;
                    }
                }

                costs.insert(new_state.clone(), new_cost);
                if !new_state.keys.iter().any(|x| !x) {
                    if new_cost < total {
                        total = new_cost;
                        println!("{}", new_cost);
                    }
                    continue;
                }

                let mut estimate : i32 = new_cost;
                let mut bad = true;
                for k in 0..26 {
                    if new_state.keys[k] {
                        continue;
                    }

                    let mut es = 0;
                    for ii in 0..3 {
                        let e = graph[new_state.robots[ii]][k].steps;
                        if e == 0 {
                            continue;
                        }
                        es = std::cmp::max(es, e);
                    }
                    if es == 0 {
                        es = col * 3;
                    } else {
                        bad = false;
                    }
                    estimate += es;
                }
                if bad {
                    continue;
                }
                while pending.contains_key(&estimate) {
                    estimate += 1;
                }
                pending.insert(estimate, new_state.clone());
            }
        }
    }
    println!("--{}--", total);
}

