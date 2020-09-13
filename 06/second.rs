use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn lineup<'a>(connections: &'a HashMap<String, String>, node: &'a str) -> Vec<&'a str> {
    let mut prev = node;
    let mut line: Vec<&str> = Vec::new();

    while prev != "COM" {
        prev = connections.get(prev).unwrap();
        line.insert(0, prev);
    }

    return line;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut connections: HashMap<String, String> = HashMap::new();

    let f = File::open("input.txt")?;
    for raw_line in BufReader::new(f).lines() {
        if let Ok(line) = raw_line {
            let parts = line.trim().split(")").collect::<Vec<&str>>();
            connections.insert(String::from(parts[1]), String::from(parts[0]));
        }
    }

    let you = lineup(&connections, "YOU");
    let san = lineup(&connections, "SAN");

    let mut count = 0usize;
    while you[count] == san[count] {
        count += 1;
    }

    println!("{}", you.len() + san.len() - count - count);

    return Ok(());
}
