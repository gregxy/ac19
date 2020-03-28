use std::io::BufRead;

fn main() {
  let mut sum : i32 = 0;
  for line in std::io::stdin().lock().lines() {
    let mut fuel = line.unwrap().parse::<i32>().unwrap() / 3 - 2;
    while fuel > 0 {
      sum += fuel;
      fuel = fuel / 3 - 2;
    }
  }
  println!("{}", sum)
}
