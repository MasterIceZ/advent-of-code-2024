use std::io::BufRead;

fn part_1(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let input = std::fs::File::open(input_file_name).unwrap();

  println!("PART 1: {}", 1);
}

fn part_2(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let input = std::fs::File::open(input_file_name).unwrap();

  println!("PART 2: {}", 1);
}

fn main() {
  let input_file_name = std::env::args().nth(1).unwrap();
  
  part_1(&input_file_name);
  println!("====================");
  part_2(&input_file_name);
}