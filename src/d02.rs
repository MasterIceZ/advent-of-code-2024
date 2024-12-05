use std::io::BufRead;

fn parse_input(line: &str) -> Vec<u32> {
  let mut res: Vec<u32> = Vec::new();
  
  for ele in line.split_whitespace() {
    res.push(ele.parse().unwrap());
  }

  return res;
}

fn work_part_1(arr: &Vec<u32>) -> u32 {
  let mut decreasing = true;
  let mut increasing = true;

  let len = arr.len();
  for i in 1..len {
    let diff = arr[i].abs_diff(arr[i - 1]);
    if diff > 3 {
      return 0;
    }
    decreasing &= arr[i] < arr[i - 1];
    increasing &= arr[i] > arr[i - 1];
  }
  if decreasing ^ increasing {
    return 1;
  }
  return 0;
}

fn work_part_2(arr: &Vec<u32>) -> u32 {
  let mut ok = work_part_1(arr) == 1;

  let len = arr.len();
  for i in 0..len {
    let mut arr_2 = arr.clone();
    arr_2.remove(i);
    ok |= work_part_1(&arr_2) == 1;
  }

  return ok as u32;
}

fn part_1(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let input = std::fs::File::open(input_file_name).unwrap();

  let mut ok_count: u32 = 0;
  for line in std::io::BufReader::new(input).lines() {
    let cur_line = parse_input(&line.unwrap());

    ok_count += work_part_1(&cur_line);
  }

  println!("PART 1: {}", ok_count);
}

fn part_2(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let input = std::fs::File::open(input_file_name).unwrap();

  let mut ok_count: u32 = 0;
  for line in std::io::BufReader::new(input).lines() {
    let cur_line = parse_input(&line.unwrap());

    ok_count += work_part_2(&cur_line);
  }

  println!("PART 2: {}", ok_count);
}

fn main() {
  let input_file_name = std::env::args().nth(1).unwrap();
  
  part_1(&input_file_name);
  println!("====================");
  part_2(&input_file_name);
}