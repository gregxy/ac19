use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut mem = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    mem[1] = 12;
    mem[2] = 2;

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
            _ => return Err(format!("unexpected op code at {} = {}", index, op).into()),
        }
        index = index + 4;
    }

    println!("{}", mem[0]);

    return Ok(());
}
