use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let prog = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut mem = prog.to_vec();
            mem[1] = noun;
            mem[2] = verb;

            if run(&mut mem) == 19690720 {
                println!("{}", noun * 100 + verb);
                return Ok(());
            }
        }
    }

    return Err("Cannot find answer".into());
}

fn run(mem: &mut Vec<i32>) -> i32 {
    let mut index: usize = 0;
    loop {
        let op = mem[index];
        let x = mem[index + 1] as usize;
        let y = mem[index + 2] as usize;
        let z = mem[index + 3] as usize;
        match op {
            1 => mem[z] = mem[x] + mem[y],
            2 => mem[z] = mem[x] * mem[y],
            99 => break,
            _ => return 0,
        }
        index = index + 4;
    }

    return mem[0];
}
