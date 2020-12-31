use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;

fn main() {
    let f = File::open("input.txt").unwrap();
    
    let mut maze : HashSet<(i32, i32)> = HashSet::new();
    let mut letters : HashMap<(i32, i32), char> = HashMap::new();

    let mut col: i32 = 0;
    for raw_line in BufReader::new(f).lines() {
        for (row, c) in raw_line.unwrap().chars().enumerate() {
            if c == '.' {
                maze.insert((row as i32, col));
            } else if c >= 'A' && c <= 'Z' {
                letters.insert((row as i32, col), c);
            }
        }
        col += 1;
    }

    let mut start_x : i32 = 0;
    let mut start_y : i32 = 0;
    let mut end_x : i32 = 0;
    let mut end_y : i32 = 0;

    // mark the teleports
    let mut teleports : HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut tracker : HashMap<(char, char), (i32, i32)> = HashMap::new();
    for (&(x, y), &c) in letters.iter() {
        let mut xx : i32 = 0;
        let mut yy : i32 = 0;
        let mut c0 : char = '@';
        let mut c1 : char = '@';
        if maze.contains(&(x+1,y)) {
            c0 = *letters.get(&(x-1, y)).unwrap();
            c1 = c;
            xx = x + 1;
            yy = y;
        } else if maze.contains(&(x-1,y)) {
            c1 = *letters.get(&(x+1, y)).unwrap();
            c0 = c;
            xx = x - 1;
            yy = y;
       } else if maze.contains(&(x,y+1)) {
            c0 = *letters.get(&(x, y-1)).unwrap();
            c1 = c;
            xx = x;
            yy = y+1;
        } else if maze.contains(&(x,y-1)) {
            c1 = *letters.get(&(x, y+1)).unwrap();
            c0 = c;
            xx = x;
            yy = y-1;
        }

        if c0 == '@' {
            continue;
        }

        if c0 == 'A' && c1 == 'A' {
            start_x = xx;
            start_y = yy;
        } else if c0 == 'Z' && c1 == 'Z' {
            end_x = xx;
            end_y = yy;
        } else {
            if let Some(&(ox, oy)) = tracker.get(&(c0, c1)) {
                teleports.insert((ox, oy), (xx, yy));
                teleports.insert((xx, yy), (ox, oy));
            } else {
                tracker.insert((c0, c1), (xx, yy));
            }
        }
    }

    let deltas = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];    

    let mut costs: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pending: BTreeMap<i32, (i32, i32)> = BTreeMap::new();
    
    costs.insert((start_x, start_y), 0);
    pending.insert(0, (start_x, start_y));

    let mut total : i32 = std::i32::MAX;
    while !pending.is_empty() {
        let (est_tmp, xy_tmp) = pending.iter().next().unwrap();
        let est = est_tmp.clone();
        let xy = xy_tmp.clone();
        pending.remove(&est);
        
        let new_cost = *costs.get(&xy).unwrap() + 1;

        for delta in deltas.iter() {
            let mut new_x = delta.0 + xy.0;
            let mut new_y = delta.1 + xy.1;

            if delta.0 == 0 && delta.1 == 0 {
                if let Some(&(xx, yy)) = teleports.get(&(new_x, new_y)) {
                    new_x = xx;
                    new_y = yy;
                } else {
                    continue;
                }
            }

            if !maze.contains(&(new_x, new_y)) {
                continue;
            }

            if let Some(&old_cost) = costs.get(&(new_x, new_y)) {
                if old_cost < new_cost {
                    continue;
                }
            }

            costs.insert((new_x, new_y).clone(), new_cost.clone());

            if new_x == end_x && new_y == end_y {
                if total > new_cost {
                    println!("{}", new_cost);
                    total = new_cost;
                }

                continue;
            }

            let mut estimate : i32 = new_cost + (new_x - end_x).abs() + 
                (new_y - end_y).abs();

            while pending.contains_key(&estimate) {
                estimate += 1;
            }

            pending.insert(estimate, (new_x, new_y));
        }
    }
}

