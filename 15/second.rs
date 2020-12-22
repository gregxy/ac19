use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let prog = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // North, South, West, East
    let frontier : Vec<(i64, i64)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
    let retrieve : Vec<i64> = vec![2, 1, 4, 3];

    let mut routes: HashMap<(i64, i64), Vec<i64>> = HashMap::new();
    routes.insert((0, 0), Vec::new());

    let mut pending: VecDeque<(i64, i64)> = VecDeque::new();
    pending.push_back((0, 0));
  
    let mut walls : HashSet<(i64, i64)> = HashSet::new();
    let mut filled : HashSet<(i64, i64)> = HashSet::new();
    let mut center : (i64, i64) = (0, 0);

    while !pending.is_empty() {
        let point = pending.pop_front().unwrap();

        let route = routes.get(&point).unwrap().clone();
        let mut computer = Computer::new(&prog);
        for step in route.iter() {
           assert_ne!(computer.run(*step), 0);
        }

        for i in 0..4 {
            let next_point : (i64, i64) =
                (frontier[i].0 + point.0, frontier[i].1 + point.1);
            if routes.contains_key(&next_point) {
                continue;
            }

            let result = computer.run((i + 1) as i64);
            if result == 0 {
                walls.insert(next_point);
                routes.insert(next_point, Vec::new());
                continue;
            }

            if result == 2 {
                center = next_point;
                println!("{:?}", center);
            }

            let mut new_route = route.clone();
            new_route.push((i+1) as i64);
            routes.insert(next_point, new_route);
            pending.push_back(next_point);
            assert_eq!(computer.run(retrieve[i]), 1);
        }
    }

    println!("{}", walls.len());

    filled.insert(center);
    let mut to_explore : Vec<(i64, i64)> = Vec::new();
    to_explore.push(center);

    let mut count : i32 = 0;
    loop {
        let mut new_explore : Vec<(i64, i64)> = Vec::new();
        for point in to_explore.iter() {
            for i in 0..4 {
                let next_point : (i64, i64) =
                    (frontier[i].0 + point.0, frontier[i].1 + point.1);
                if walls.contains(&next_point) ||
                    filled.contains(&next_point) {
                    continue;
                }
                filled.insert(next_point);
                new_explore.push(next_point);
            }
        }

        if new_explore.is_empty() {
            break;
        }
        count += 1;
        to_explore.clone_from(&new_explore);
    }

    println!("{}", count);
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
