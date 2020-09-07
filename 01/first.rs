use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let sum = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap() / 3 - 2)
        .sum::<i32>();

    println!("{}", sum);

    return Ok(());
}
