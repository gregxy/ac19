use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn cal(v: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut fuel = v / 3 - 2;
    while fuel > 0 {
        sum += fuel;
        fuel = fuel / 3 - 2;
    }

    return sum;
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let sum = BufReader::new(f)
        .lines()
        .map(|line| cal(line.unwrap().parse::<i32>().unwrap()))
        .sum::<i32>();

    println!("{}", sum);

    return Ok(());
}
