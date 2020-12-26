use std::collections::HashMap;

fn main() {
    let mut prog = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    prog[0] = 2;
    let mut computer = Computer::new(&prog);
    //A, B, B, A, C, B, C, C, B, A<LF>
    let main : Vec<i64> = vec![
        65, 44,
        66, 44,
        66, 44,
        65, 44,
        67, 44,
        66, 44,
        67, 44,
        67, 44,
        66, 44,
        65, 10];

    // R,10, R,8, L,10, L,10 <LF>
    let a_prog : Vec<i64> = vec![
        82, 44, 49, 48, 44,
        82, 44, 56, 44,
        76, 44, 49, 48, 44,
        76, 44, 49, 48, 10];
    
    // R,8, L6, L6
    let b_prog : Vec<i64> = vec![
        82, 44, 56, 44,
        76, 44, 54, 44,
        76, 44, 54, 10];

    // L,10,R,10,L,6
    let c_prog : Vec<i64> = vec![
        76, 44, 49, 48, 44,
        82, 44, 49, 48, 44,
        76, 44, 54, 10];

    let tail : Vec<i64> = vec![
        110, 10];

    let mut inputs : Vec<i64> = Vec::new();
    inputs.extend(main.iter());
    inputs.extend(a_prog.iter());
    inputs.extend(b_prog.iter());
    inputs.extend(c_prog.iter());
    inputs.extend(tail.iter());

    computer.run(&inputs);

    println!("{}", inputs.len());
}

struct Computer {
    index: usize,
    base: i64,
    mem: HashMap<usize, i64>,
}

impl Computer {
    fn new(prog: &Vec<i64>) -> Computer {
        let mut computer = Computer {
            index: 0,
            base: 0,
            mem: HashMap::new(),
        };

        for (index, v) in prog.iter().enumerate() {
            computer.mem.insert(index, *v);
        }

        return computer;
    }

    fn fetch(&self, address: usize) -> i64 {
        if let Some(v) = self.mem.get(&address) {
            return *v;
        }

        return 0i64;
    }

    fn decode(&self) -> (i64, usize, usize, usize) {
        let opv = self.fetch(self.index);

        let c = (opv / 100) % 10;
        let d = (opv / 1000) % 10;
        let e = (opv / 10000) % 10;
        let op = opv % 100;

        let la = if c == 1 {
            self.index + 1
        } else if c == 0 {
            self.fetch(self.index + 1) as usize
        } else {
            (self.base + self.fetch(self.index + 1)) as usize
        };

        let ra = if d == 1 {
            self.index + 2
        } else if d == 0 {
            self.fetch(self.index + 2) as usize
        } else {
            (self.base + self.fetch(self.index + 2)) as usize
        };

        let ta = if e == 1 {
            self.index + 3
        } else if e == 0 {
            self.fetch(self.index + 3) as usize
        } else {
            (self.base + self.fetch(self.index + 3)) as usize
        };

        return (op, la, ra, ta);
    }

    fn run(&mut self, inputs : &Vec<i64>) -> i64 {
        let mut p : usize = 0;
        loop {
            let (op, la, ra, ta) = self.decode();

            match op {
                1 | 2 => {
                    if op == 1 {
                        self.mem.insert(ta, self.fetch(la) + self.fetch(ra));
                    } else {
                        self.mem.insert(ta, self.fetch(la) * self.fetch(ra));
                    }
                    self.index += 4;
                }
                3 => {
                    self.mem.insert(la, inputs[p]);
                    p += 1;
                    self.index += 2;
                }
                4 => {
                    let out = self.fetch(la);
                    self.index += 2;
                    if out <= 127 {
                        print!("{}", char::from(out as u8));
                    } else {
                        println!("\n--{}--", out);
                    }
                }
                5 | 6 => {
                    self.index = match (op == 5, self.fetch(la) != 0) {
                        (true, true) => self.fetch(ra) as usize,
                        (true, false) => self.index + 3,
                        (false, true) => self.index + 3,
                        (false, false) => self.fetch(ra) as usize,
                    }
                }
                7 | 8 => {
                    if op == 7 {
                        let v: i64 = if self.fetch(la) < self.fetch(ra) {
                            1
                        } else {
                            0
                        };
                        self.mem.insert(ta, v);
                    } else {
                        let v: i64 = if self.fetch(la) == self.fetch(ra) {
                            1
                        } else {
                            0
                        };
                        self.mem.insert(ta, v);
                    }
                    self.index += 4;
                }
                9 => {
                    self.base += self.fetch(la);
                    self.index += 2;
                }
                99 => break,
                _ => panic!("Unexpected op code"),
            }
        }
        println!("{}", p);
        return -1;
    }
}
