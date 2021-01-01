use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn cut(cards : &mut Vec<i64>, n : i64) {
    if n >= 0 {
        cards.rotate_left(n as usize);
    } else {
        cards.rotate_right((-n) as usize);
    }
}

fn deal_inc(cards : &Vec<i64>, n : usize) -> Vec<i64> {
    let mut new_cards : Vec<i64> = vec![0; cards.len()];
    
    let mut index : usize = 0;
    for value in cards.iter() {
        new_cards[index] = *value;
        index += n;
        if index >= cards.len() {
            index = index - cards.len();
        }
    }

    return new_cards;
}

fn main() {
    let mut cards : Vec<i64> = Vec::new();
    for i in 0..10007 {
        cards.push(i as i64);
    }

    let f = File::open("input.txt").unwrap();
    for raw_line in BufReader::new(f).lines() {
        let line = raw_line.unwrap().trim().to_string();
        if line.starts_with("deal into new stack") {
            cards.reverse();
        } else if let Some(n) = line.strip_prefix("deal with increment ") {
            let nn = n.parse::<usize>().unwrap();
            cards = deal_inc(&cards, nn);
        } else if let Some(n) = line.strip_prefix("cut ") {
            let nn = n.parse::<i64>().unwrap();
            cut(&mut cards, nn);
        } else {
            panic!(line);
        }
    }

    println!("{}", cards.iter().position(|&x| x == 2019).unwrap());
}
