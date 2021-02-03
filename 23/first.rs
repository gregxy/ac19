use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let prog = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
        
    let mut computers : Vec<Computer> = Vec::new();
    let mut queues : Vec<VecDeque<i64>> = Vec::new();
    for i in 0..50 {
        computers.push(Computer::new(&prog));
        queues.push(VecDeque::new());
        queues[i].push_back(i as i64);
    }

    let mut iteration : i64 = 0;
    loop {
        for i in 0..50 {
            let addr = computers[i].run(&mut queues[i]);
            if addr < 0 {
                continue;
            }
            let x = computers[i].run(&mut queues[i]);
            let y = computers[i].run(&mut queues[i]);

            println!("{} {}: <{}, {}, {}>", iteration, i, addr, x, y);
            if addr == 255 {
                println!("{}", y);
                return;
            }

            queues[addr as usize].push_back(x);
            queues[addr as usize].push_back(y);
        }
        iteration += 1;
    }
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

    fn run(&mut self, inputs : &mut VecDeque<i64>) -> i64 {
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
                    self.index += 2;
                    if let Some(v) = inputs.pop_front() {
                        self.mem.insert(la, v);
                    } else {
                        self.mem.insert(la, -1);                    
                        return -1;
                    }
                }
                4 => {
                    let out = self.fetch(la);
                    self.index += 2;
                    return out;
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
        return -1;
    }
}
