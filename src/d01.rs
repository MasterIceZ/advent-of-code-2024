use std::io::BufRead;
use std::collections::HashMap;

fn parse_input(line: &str) -> (u32, u32) {
  let (l, r) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();

  let l = l.parse().unwrap();
  let r = r.trim_ascii_start().parse().unwrap();

  return (l, r);
}

fn part_1(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let mut left_arr: Vec<u32> = Vec::new();
  let mut right_arr: Vec<u32> = Vec::new();

  let input = std::fs::File::open(input_file_name).unwrap();

  for line in std::io::BufReader::new(input).lines() {
    let (l, r) = parse_input(&line.unwrap());
    left_arr.push(l);
    right_arr.push(r);
  }

  left_arr.sort();
  right_arr.sort();

  let len = left_arr.len();
  let mut sum: u32 = 0;
  for i in 0..len {
    sum += left_arr[i].abs_diff(right_arr[i]);
  }

  println!("PART 1: {}", sum);
}

fn part_2(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let mut left_arr: Vec<u32> = Vec::new();
  let mut right_hash_table: HashMap<u32, u32> = HashMap::new();

  let input = std::fs::File::open(input_file_name).unwrap();

  for line in std::io::BufReader::new(input).lines() {
    let (l, r) = parse_input(&line.unwrap());
    left_arr.push(l);

    let count = right_hash_table.entry(r).or_insert(0);
    *count += 1;
  }

  let len = left_arr.len();
  let mut sum: u32 = 0;
  for i in 0..len {
    if !right_hash_table.contains_key(&left_arr[i]) {
      continue;
    }
    let count = right_hash_table.get_mut(&left_arr[i]).unwrap();
    sum += left_arr[i] * (*count);
  }

  println!("PART 2: {}", sum);
}

fn main() {
  let input_file_name = std::env::args().nth(1).unwrap();
  
  part_1(&input_file_name);
  println!("====================");
  part_2(&input_file_name);
}