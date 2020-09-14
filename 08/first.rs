use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let length = 25 * 6;
    let serialized = fs::read_to_string("input.txt")?;
    let mut m = std::i32::MAX;
    let mut mv = 0;

    let mut zeros: i32 = 0;
    let mut ones: i32 = 0;
    let mut twos: i32 = 0;
    let mut index: i32 = 0;
    for ch in serialized.chars() {
        match ch {
            '0' => zeros += 1,
            '1' => ones += 1,
            '2' => twos += 1,
            _ => (),
        }
        index += 1;
        if index % length == 0 {
            if zeros < m {
                mv = ones * twos;
                m = zeros;
            }
            zeros = 0;
            ones = 0;
            twos = 0;
        }
    }

    println!("{}", mv);

    return Ok(());
}
