use std::collections::HashMap;
use std::collections::HashSet;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

fn add(sb: &mut HashMap<(i32, i32), Vec<(i32, i32)>>, x: i32, y: i32, xx: i32, yy: i32) {
    if let Some(v) = sb.get_mut(&(x, y)) {
        v.push((xx, yy));
    } else {
        sb.insert((x, y), vec![(xx, yy)]);
    }
}

fn sortv(lhs: &(i32, i32), rhs: &(i32, i32)) -> std::cmp::Ordering {
    let lv = (lhs.0 - 22).abs() + (lhs.1 - 28).abs();
    let rv = (rhs.0 - 22).abs() + (rhs.1 - 28).abs();

    return lv.cmp(&rv);
}

fn phase(dx: i32, dy: i32) -> i32 {
    if dx == 0 {
        if dy < 0 {
            return 0;
        } else {
            return 4;
        }
    }

    if dy == 0 {
        if dx < 0 {
            return 6;
        } else {
            return 2;
        }
    }

    if dy < 0 && dx > 0 {
        return 1;
    }

    if dy > 0 && dx > 0 {
        return 3;
    }

    if dx < 0 && dy > 0 {
        return 5;
    }

    return 7;
}

fn sortk(l: &(i32, i32), r: &(i32, i32)) -> std::cmp::Ordering {
    let pl = phase(l.0, l.1);
    let pr = phase(r.0, r.1);

    if pl != pr {
        return pl.cmp(&pr);
    }

    let tl = ((l.0 as f64) / (l.1 as f64)).abs();
    let tr = ((r.0 as f64) / (r.1 as f64)).abs();
    if pl == 1 || pl == 5 {
        if tr > tl {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }

    if tl > tr {
        return std::cmp::Ordering::Less;
    } else {
        return std::cmp::Ordering::Greater;
    }
}

fn main() {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    let content = std::fs::read_to_string("input.txt").unwrap();
    for (y, line) in content.lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            if ch == '.' {
                continue;
            }

            if ch == '#' {
                grid.insert((x as i32, y as i32));
            }
        }
    }

    let x: i32 = 22;
    let y: i32 = 28;
    grid.remove(&(x, y));
    let mut sb: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for (xx, yy) in grid.iter() {
        let dx = xx - x;
        let dy = yy - y;

        let adx = dx.abs();
        let ady = dy.abs();

        let g: i32;
        if dx == 0 {
            g = ady;
        } else if dy == 0 {
            g = adx;
        } else {
            g = if adx > ady {
                gcd(adx, ady)
            } else {
                gcd(ady, adx)
            };
        }

        add(&mut sb, dx / g, dy / g, *xx, *yy);
    }

    for (_, val) in sb.iter_mut() {
        val.sort_by(|a, b| sortv(a, b));
    }

    let mut ord: Vec<(i32, i32)> = Vec::new();
    for k in sb.keys() {
        ord.push(*k);
    }

    ord.sort_by(|a, b| sortk(a, b));
    let mut count = 0;
    let mut index = 0;
    while count <= 200 {
        if index == ord.len() {
            index = 0;
        }

        let v = sb.get_mut(&ord[index]).unwrap();
        if v.len() != 0 {
            count += 1;
            let t = v.remove(0);
            println!("{}: {} {}", count, t.0, t.1);
        }
        index += 1;
    }
}
