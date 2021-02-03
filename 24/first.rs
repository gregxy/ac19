use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let f = File::open("input.txt").unwrap();

    let mut bugs : [[[bool; 5]; 5]; 2] = [[[false; 5];5];2];
    let mut current : usize = 0;
    let mut next : usize = 1;

    let mut col: i32 = 0;
    for raw_line in BufReader::new(f).lines() {
        for (row, c) in raw_line.unwrap().chars().enumerate() {
            if c == '#' {
                bugs[current][col as usize][row as usize] = true;
            } 
        }
        col += 1;
    }

    let mut sb : HashSet<u64> = HashSet::new();

    let deltas : [(i32, i32); 4] = [(0,1), (0,-1), (1,0), (-1,0)];
    loop {
        for i in 0..5usize {
            for j in 0..5usize {
                bugs[next][i][j] = false;
            }
        }

        for i in 0..5usize {
            for j in 0..5usize {
                let mut count : i32 = 0;

                for delta in deltas.iter() {
                    let ii : i32 = i as i32 + delta.0;
                    let jj : i32 = j as i32 + delta.1;

                    if ii < 0 || jj < 0 || ii > 4 || jj > 4 {
                        continue;
                    }

                    if bugs[current][ii as usize][jj as usize] {
                        count += 1;
                    }
                }

                if bugs[current][i][j] {
                    if count == 1 {
                        bugs[next][i][j] = true;
                    }
                } else {
                    if count == 1 || count == 2 {
                        bugs[next][i][j] = true;
                    }
                }
           }
        }
        let mut sum : u64 = 0;
        let mut p : u64 = 1;
        for i in 0..5usize {
            for j in 0..5usize {
               if bugs[next][i as usize][j as usize] {
                   sum += p;
               }
               p = p << 1;
           }
        }
        if sb.contains(&sum) {
            println!("{}", sum);
            return;
        }
        sb.insert(sum);

        if current == 0 {
            current = 1;
            next = 0;
        } else {
            current = 0;
            next = 1;
        }

    }
}
