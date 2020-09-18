use std::collections::HashSet;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
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

    let mut count: usize = 0;
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;

    for (x, y) in grid.iter() {
        let mut sb: HashSet<(i32, i32)> = HashSet::new();
        for (xx, yy) in grid.iter() {
            if xx == x && yy == y {
                continue;
            }

            if xx == x {
                if y > yy {
                    sb.insert((0, 1));
                } else {
                    sb.insert((0, -1));
                }
                continue;
            }

            if yy == y {
                if x > xx {
                    sb.insert((1, 0));
                } else {
                    sb.insert((-1, 0));
                }
                continue;
            }

            let xv = xx - x;
            let yv = yy - y;

            let xvv = xv.abs();
            let yvv = yv.abs();

            let g = if xvv > yvv {
                gcd(xvv, yvv)
            } else {
                gcd(yvv, xvv)
            };

            sb.insert((xv / g, yv / g));
        }
        if sb.len() > count {
            count = sb.len();
            cx = *x;
            cy = *y;
        }
    }

    println!("{} ({}, {})", count, cx, cy);
}
