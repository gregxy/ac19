use permutator::Permutation;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let prog = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut data = vec![0, 1, 2, 3, 4];
    let mut v = 0i32;
    let mut last = 0i32;
    data.permutation().for_each(|perm| {
        last = 0;
        for p in perm {
            let mut mem = prog.to_vec();
            last = run(&mut mem, &[p, last]);
        }
        v = std::cmp::max(v, last);
    });

    println!("{}", v);

    return Ok(());
}

fn get_addresses(mem: &Vec<i32>, index: usize) -> (usize, usize) {
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

    return (la, ra);
}

fn run(mem: &mut Vec<i32>, input: &[i32; 2]) -> i32 {
    let mut index: usize = 0;
    let mut out: i32 = 0;
    let mut ipos = 0usize;
    loop {
        let op = mem[index] % 100;
        match op {
            1 | 2 => {
                let (la, ra) = get_addresses(mem, index);
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
                mem[la] = input[ipos];
                ipos += 1;
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
                index += 2;
            }
            5 | 6 => {
                let (la, ra) = get_addresses(mem, index);

                index = match (op == 5, mem[la] != 0) {
                    (true, true) => mem[ra] as usize,
                    (true, false) => index + 3,
                    (false, true) => index + 3,
                    (false, false) => mem[ra] as usize,
                }
            }
            7 | 8 => {
                let (la, ra) = get_addresses(mem, index);
                let z = mem[index + 3] as usize;

                if op == 7 {
                    mem[z] = if mem[la] < mem[ra] { 1 } else { 0 };
                } else {
                    mem[z] = if mem[la] == mem[ra] { 1 } else { 0 };
                }
                index += 4;
            }
            99 => break,
            _ => panic!("Unexpected op code"),
        }
    }

    return out;
}
