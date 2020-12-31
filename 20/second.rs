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

    const EX_X0 : i32 = 2;
    const EX_Y0 : i32 = 2;
    const EX_X1 : i32 = 126;
    const EX_Y1 : i32 = 132;

    let mut start_x : i32 = 0;
    let mut start_y : i32 = 0;
    let mut end_x : i32 = 0;
    let mut end_y : i32 = 0;

    // mark the teleports
    let mut teleports : HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut tracker : HashMap<(char, char), (i32, i32)> = HashMap::new();
    let mut external : HashSet<(i32, i32)> = HashSet::new();
    let mut internal : HashSet<(i32, i32)> = HashSet::new();
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
            if xx == EX_X0 || xx == EX_X1 ||
                yy == EX_Y0 || yy == EX_Y1 {
                external.insert((xx, yy));
            } else {
                internal.insert((xx, yy));
            }

            if let Some(&(ox, oy)) = tracker.get(&(c0, c1)) {
                teleports.insert((ox, oy), (xx, yy));
                teleports.insert((xx, yy), (ox, oy));
            } else {
                tracker.insert((c0, c1), (xx, yy));
            }
        }
    }

    let deltas = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];    

    let mut costs: HashMap<(i32, i32, i32), i32> = HashMap::new();
    let mut pending: BTreeMap<i32, (i32, i32, i32)> = BTreeMap::new();
    
    costs.insert((start_x, start_y, 0), 0);
    pending.insert(0, (start_x, start_y, 0));
    let mut found : bool = false;
    while !pending.is_empty() {
        let (est_tmp, xyz_tmp) = pending.iter().next().unwrap();
        let est = est_tmp.clone();
        let xyz = xyz_tmp.clone();
        pending.remove(&est);
        
        let new_cost = *costs.get(&xyz).unwrap() + 1;
        for delta in deltas.iter() {
            let mut new_x = delta.0 + xyz.0;
            let mut new_y = delta.1 + xyz.1;
            let mut new_z = xyz.2;

            if delta.0 == 0 && delta.1 == 0 {
                if let Some(&(xx, yy)) = teleports.get(&(new_x, new_y)) {
                    if external.contains(&(new_x, new_y)) {
                        if new_z == 0 {
                            continue;
                        }
                        new_z -= 1;
                    } else {
                        assert!(internal.contains(&(new_x, new_y)));
                        new_z += 1;
                    }

                    new_x = xx;
                    new_y = yy;
                } else {
                    continue;
                }
            }

            if !maze.contains(&(new_x, new_y)) {
                continue;
            }

            if let Some(&old_cost) = costs.get(&(new_x, new_y, new_z)) {
                if old_cost < new_cost {
                    continue;
                }
            }

            costs.insert((new_x, new_y, new_z), new_cost);

            if new_x == end_x && new_y == end_y && new_z == 0{
                println!("{}", new_cost);
                found = true;
                break;
            }

            let mut estimate : i32 = new_cost + (new_x - end_x).abs() + 
                (new_y - end_y).abs();

            if new_z >= 1 {
                let mut ee : i32 = 0;
                for (nx, ny) in external.iter() {
                    ee = std::cmp::max(ee, 
                        (nx - new_x).abs() + (ny - new_y).abs());
                }
                estimate += ee;
            }

            if new_z >= 2 {
                estimate += (new_z - 1) * 258;
            }

            while pending.contains_key(&estimate) {
                estimate += 1;
            }

            pending.insert(estimate, (new_x, new_y, new_z));
        }
        if found {
            break;
        }
    }
}

