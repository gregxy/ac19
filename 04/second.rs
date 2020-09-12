fn explore(pos: i32, sofar: i32, last: i32, min: i32, max: i32, cs: bool, n: bool) -> i32 {
    let mut count: i32 = 0;

    for i in last..=9 {
        let s = sofar + i * pos;
        if s > max {
            continue;
        }

        let ncs: bool;
        let nn: bool;
        match (cs, n) {
            (true, true) => {
                if i == last {
                    nn = true;
                    ncs = false;
                } else {
                    ncs = true;
                    nn = false;
                }
            }
            (true, false) => {
                ncs = true;
                nn = false;
            }
            (false, false) => {
                ncs = last == i;
                nn = ncs;
            }
            (false, true) => {
                ncs = false;
                nn = i == last;
            }
        }

        if pos != 1 {
            count += explore(pos / 10, s, i, min, max, ncs, nn);
        } else if ncs && s >= min && s <= max {
            count += 1;
        }
    }

    return count;
}

fn main() {
    //272091 - 815432
    let mut count: i32 = 0;
    for i in 2..=8 {
        count += explore(10000, i * 100000, i, 272091, 815432, false, false);
    }

    println!("{}", count);
}
