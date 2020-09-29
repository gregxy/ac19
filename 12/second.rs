use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

fn encode(s: &(i32, i32, i32, i32, i32, i32, i32, i32)) -> u64 {
    let mut hs = DefaultHasher::new();
    s.0.hash(&mut hs);
    s.1.hash(&mut hs);
    s.2.hash(&mut hs);
    s.3.hash(&mut hs);
    s.4.hash(&mut hs);
    s.5.hash(&mut hs);
    s.6.hash(&mut hs);
    s.7.hash(&mut hs);

    return hs.finish();
}

fn main() {
    let mut p: [[i32; 3]; 4] = [[5, -1, 5], [0, -14, 2], [16, 4, 0], [18, 1, 16]];
    let mut v: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut result: [u64; 3] = [0; 3];
    for i in 0..3 {
        let mut state: HashSet<u64> = HashSet::new();
        loop {
            let f = (
                p[0][i], p[1][i], p[2][i], p[3][i], v[0][i], v[1][i], v[2][i], v[3][i],
            );
            let s = encode(&f);
            if state.contains(&s) {
                println!("Done {}: {}", i, result[i]);
                break;
            }
            state.insert(s);

            if result[i] % 1000000 == 0 {
                println!("Iterations: {}", result[i]);
            }
            result[i] += 1;

            for x in 0..3 {
                for y in x + 1..4 {
                    if p[x][i] > p[y][i] {
                        v[x][i] -= 1;
                        v[y][i] += 1;
                    } else if p[x][i] < p[y][i] {
                        v[x][i] += 1;
                        v[y][i] -= 1;
                    }
                } // y in 0 ..4
            } // x in 0..4

            for x in 0..4 {
                p[x][i] += v[x][i];
            }
        }
    }
    println!("`{:?}", result)
}
