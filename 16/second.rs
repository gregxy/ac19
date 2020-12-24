fn main() {
    let signal = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    
    let true_length = signal.len() * 10000;
    let length = signal.len();

    let mut offset : usize = 0;
    for i in 0..7 {
        offset = offset * 10 + (signal[i] as usize);
    }

    let delta_length = true_length - offset;

    let mut mem: Vec<i32> = Vec::new();
    for i in offset..true_length {
        mem.push(signal[i % length]);
    }

    for x in 0..100 {
        println!("iteration: {}", x);

        let mut v : i32 = mem.iter().sum();
        let mut last : i32 = 0;
        for i in 0..delta_length {
            v = v - last;
            last = mem[i];

            let mut vv : i32 = v;
            if vv < 0 {
                vv = -vv;
            }
            mem[i] = vv % 10;
        }
    }

    for i in 0..8 {
        print!("{}", mem[i]);
    }
}
