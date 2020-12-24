fn calculate_patterns(length : usize) -> Vec<Vec<i32>> {
    let mut patterns : Vec<Vec<i32>> = Vec::new();
    let base = vec![0, 1, 0, -1];

    for p in 1..(length + 1) {
        let mut current : Vec<i32> = Vec::new();
        let mut bp : usize = 0;
        let mut sp : usize = 0;

        for _ in 0..length+1 {
            current.push(base[bp]);
            sp += 1;

            if sp == p {
                sp = 0;
                bp = (bp + 1) % 4;
            }
        }
        current[0] = if current.iter().any(|&x| x != 0) {
            1
        } else {
            0
        };

        patterns.push(current);
    }

    return patterns;
}

fn main() {
    let signal = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let length = signal.len();
    let patterns = calculate_patterns(length);

    let mut mem: Vec<Vec<i32>> = Vec::new();
    mem.push(signal.clone());
    mem.push(vec![0i32; length]);

    let mut src : usize = 0;
    let mut dst : usize = 1;

    for _ in 0..100 {
        for i in 0..length {
            mem[dst][i] = 0;
            if patterns[i][0] == 0 {
                continue;
            }

            for j in 0..length {
                mem[dst][i] += mem[src][j] * patterns[i][j+1];  
            }

            if mem[dst][i] < 0 {
                mem[dst][i] = -mem[dst][i];
            }
            mem[dst][i] = mem[dst][i] % 10;
        }

        src = if src == 0 {
            1
        } else {
            0
        };

        dst = if dst == 0 {
            1
        } else {
            0
        };
    }

    println!("{:?}", mem[src]);
}
