use std::io::BufRead;

fn main() {
  let mut sum : i32 = 0;
  for line in std::io::stdin().lock().lines() {
    let num = line.unwrap().parse::<i32>().unwrap();
    sum += num / 3 - 2
  }
  println!("{}", sum)
}
