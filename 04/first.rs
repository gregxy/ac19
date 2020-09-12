fn explore(pos: i32, sofar: i32, last: i32, min: i32, max: i32, cs: bool) -> i32 {
    let mut count: i32 = 0;
    for i in last..=9 {
        let s = sofar + i * pos;
        if s > max {
            continue;
        }

        let ncs = cs || (last == i);
        if pos != 1 {
            count += explore(pos / 10, s, i, min, max, ncs);
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
        count += explore(10000, i * 100000, i, 272091, 815432, false);
    }

    println!("{}", count);
}
