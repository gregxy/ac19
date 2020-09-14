use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let length: usize = 25 * 6;
    let serialized = fs::read_to_string("input.txt")?;
    let mut buf: [u8; 150] = [2; 150];

    for (index, ch) in serialized.chars().enumerate() {
        let pos = index % length;
        if buf[pos] != 2 {
            continue;
        }

        if ch == '1' {
            buf[pos] = 1;
        } else if ch == '0' {
            buf[pos] = 0;
        }
    }

    for i in 0..6 {
        for j in 0..25 {
            print!("{}", buf[(i * 25 + j)]);
        }
        println!("");
    }

    return Ok(());
}
