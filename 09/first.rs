use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let prog = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut mem: HashMap<usize, i64> = HashMap::new();

    for (index, v) in prog.iter().enumerate() {
        mem.insert(index, *v);
    }

    let v = run(&mut mem, 1);

    println!("{}", v);

    return Ok(());
}

fn fetch(mem: &HashMap<usize, i64>, index: usize) -> i64 {
    if let Some(v) = mem.get(&index) {
        return *v;
    }

    return 0i64;
}

fn decode(mem: &HashMap<usize, i64>, index: usize, base: i64) -> (i64, usize, usize, usize) {
    let opv = fetch(mem, index);

    let c = (opv / 100) % 10;
    let d = (opv / 1000) % 10;
    let e = (opv / 10000) % 10;
    let op = opv % 100;

    let la = if c == 1 {
        index + 1
    } else if c == 0 {
        fetch(mem, index + 1) as usize
    } else {
        (base + fetch(mem, index + 1)) as usize
    };

    let ra = if d == 1 {
        index + 2
    } else if d == 0 {
        fetch(mem, index + 2) as usize
    } else {
        (base + fetch(mem, index + 2)) as usize
    };

    let ta = if e == 1 {
        index + 3
    } else if e == 0 {
        fetch(mem, index + 3) as usize
    } else {
        (base + fetch(mem, index + 3)) as usize
    };

    return (op, la, ra, ta);
}

fn run(mem: &mut HashMap<usize, i64>, input: i64) -> i64 {
    let mut index: usize = 0;
    let mut out: i64 = 0;
    let mut base: i64 = 0;

    loop {
        let (op, la, ra, ta) = decode(mem, index, base);

        match op {
            1 | 2 => {
                if op == 1 {
                    mem.insert(ta, fetch(mem, la) + fetch(mem, ra));
                } else {
                    mem.insert(ta, fetch(mem, la) * fetch(mem, ra));
                }
                index += 4;
            }
            3 => {
                mem.insert(la, input);
                index += 2;
            }
            4 => {
                out = fetch(mem, la);
                index += 2;
            }
            5 | 6 => {
                index = match (op == 5, fetch(mem, la) != 0) {
                    (true, true) => fetch(mem, ra) as usize,
                    (true, false) => index + 3,
                    (false, true) => index + 3,
                    (false, false) => fetch(mem, ra) as usize,
                }
            }
            7 | 8 => {
                if op == 7 {
                    let v: i64 = if fetch(mem, la) < fetch(mem, ra) {
                        1
                    } else {
                        0
                    };
                    mem.insert(ta, v);
                } else {
                    let v: i64 = if fetch(mem, la) == fetch(mem, ra) {
                        1
                    } else {
                        0
                    };
                    mem.insert(ta, v);
                }
                index += 4;
            }
            9 => {
                base += fetch(mem, la);
                index += 2;
            }
            99 => break,
            _ => panic!("Unexpected op code"),
        }
    }

    return out;
}
