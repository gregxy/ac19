use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let prog = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut mem = prog.to_vec();
    println!("{}", run(&mut mem, 1));

    return Ok(());
}

fn run(mem: &mut Vec<i32>, input: i32) -> i32 {
    let mut index: usize = 0;
    let mut out: i32 = 0;
    loop {
        let op = mem[index] % 100;
        match op {
            1 | 2 => {
                let c = (mem[index] / 100) % 10;
                let d = (mem[index] / 1000) % 10;

                let la = if c == 1 {
                    index + 1
                } else {
                    mem[index + 1] as usize
                };

                let ra = if d == 1 {
                    index + 2
                } else {
                    mem[index + 2] as usize
                };

                let z = mem[index + 3] as usize;
                if op == 1 {
                    mem[z] = mem[la] + mem[ra];
                } else {
                    mem[z] = mem[la] * mem[ra];
                }
                index += 4;
            }
            3 => {
                let la = mem[index + 1] as usize;
                mem[la] = input;
                index += 2;
            }
            4 => {
                let c = (mem[index] / 100) % 10;

                let la = if c == 1 {
                    index + 1
                } else {
                    mem[index + 1] as usize
                };

                out = mem[la];
                println!("{}", out);
                index += 2;
            }
            99 => break,
            _ => panic!("Unexpected op code"),
        }
    }

    return out;
}
