use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let prog = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut floor = Floor::new();
    let mut computer = Computer::new(&prog);

    let mut dir = Direction::Up;
    let mut tiles: HashSet<(i32, i32)> = HashSet::new();
    let mut addr: (i32, i32) = (0, 0);

    loop {
        let input = floor.fetch(addr);

        let out1 = computer.run(input);
        if out1 == -1 {
            break;
        }

        let out2 = computer.run(input);

        floor.store(addr, out1);
        tiles.insert(addr);
        dir = dir.turn(Turn::from_i64(out2));
        addr = dir.step(addr);
    }

    println!("{}", tiles.len());
}

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

enum Turn {
    Left,
    Right,
}

impl Direction {
    fn turn(&self, t: Turn) -> Direction {
        match (self, t) {
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Left, Turn::Right) => Direction::Up,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
        }
    }

    fn step(&self, c: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (c.0, c.1 + 1),
            Direction::Down => (c.0, c.1 - 1),
            Direction::Left => (c.0 - 1, c.1),
            Direction::Right => (c.0 + 1, c.1),
        }
    }
}

impl Turn {
    fn from_i64(i: i64) -> Turn {
        if i == 0 {
            return Turn::Left;
        }

        return Turn::Right;
    }
}

struct Floor {
    mem: HashMap<(i32, i32), i64>,
}

impl Floor {
    fn new() -> Floor {
        Floor {
            mem: HashMap::new(),
        }
    }

    fn fetch(&self, address: (i32, i32)) -> i64 {
        if let Some(v) = self.mem.get(&address) {
            return *v;
        }

        return 0;
    }

    fn store(&mut self, address: (i32, i32), val: i64) {
        self.mem.insert(address, val);
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

    fn run(&mut self, input: i64) -> i64 {
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
                    self.mem.insert(la, input);
                    self.index += 2;
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
