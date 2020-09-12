use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn next(ori: &(i32, i32), step: &str) -> ((i32, i32), (i32, i32)) {
    let direction = step.chars().next().unwrap();
    let distance = step.get(1..).unwrap().parse::<i32>().unwrap();

    let mut x = ori.0;
    let mut y = ori.1;
    let delta: (i32, i32);
    match direction {
        'U' => {
            y += distance;
            delta = (0, 1);
        }
        'D' => {
            y -= distance;
            delta = (0, -1);
        }
        'L' => {
            x -= distance;
            delta = (-1, 0);
        }

        'R' => {
            x += distance;
            delta = (1, 0);
        }
        _ => panic!("unexpectd distance"),
    }

    return ((x, y), delta);
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.trim()
                .split(",")
                .map(|x| String::from(x))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let mut count = 0;
    for step in &lines[0] {
        let (end, delta) = next(&(pos_x, pos_y), step);

        while end.0 != pos_x || end.1 != pos_y {
            pos_x = pos_x + delta.0;
            pos_y = pos_y + delta.1;
            count += 1;
            grid.insert((pos_x, pos_y), count);
        }
    }

    let mut m = std::i32::MAX;
    pos_x = 0;
    pos_y = 0;
    count = 0;
    for step in &lines[1] {
        let (end, delta) = next(&(pos_x, pos_y), step);

        while end.0 != pos_x || end.1 != pos_y {
            pos_x += delta.0;
            pos_y += delta.1;
            count += 1;
            if let Some(c) = grid.get(&(pos_x, pos_y)) {
                m = std::cmp::min(m, c + count);
            }
        }
    }

    println!("{}", m);

    return Ok(());
}
