use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn fill<'a>(
    connections: &'a HashMap<String, Vec<String>>,
    sb: &mut HashMap<&'a str, i32>,
    node: &'a str,
    val: i32,
) {
    sb.insert(node, val);
    if let Some(dots) = connections.get(node) {
        for neighbor in dots.iter() {
            fill(connections, sb, neighbor, val + 1);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    let f = File::open("input.txt")?;
    for raw_line in BufReader::new(f).lines() {
        if let Ok(line) = raw_line {
            let parts = line.trim().split(")").collect::<Vec<&str>>();
            if let Some(dots) = connections.get_mut(parts[0]) {
                dots.push(String::from(parts[1]));
            } else {
                connections.insert(String::from(parts[0]), [String::from(parts[1])].to_vec());
            }
        }
    }

    let mut sb: HashMap<&str, i32> = HashMap::new();

    fill(&connections, &mut sb, "COM", 0);

    println!("{}", sb.values().sum::<i32>());

    return Ok(());
}
