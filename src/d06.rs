// TODO: OPTIMIZE PART 2!

use std::io::BufRead;
use std::collections::HashSet;

fn parse_input(input_file_name: &str) -> Vec<String> {
  let input = std::fs::File::open(input_file_name).unwrap();

  let mut arr: Vec<String> = Vec::new();
  for line in std::io::BufReader::new(input).lines() {
    let cur_line = line.unwrap();
    arr.push(cur_line);
  }

  return arr;
}

fn part_1(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  
  let arr = parse_input(input_file_name);
  let n = arr.len();
  let m = arr[0].len();
  
  let mut stp_i: i32 = -1;
  let mut stp_j: i32 = -1;
  for i in 0..n {
    for j in 0..m {
      if arr[i].chars().nth(j).unwrap() == '^' {
        stp_i = i as i32;
        stp_j = j as i32;
        break;
      }
    }
  }

  let di = vec![-1, 0, 1, 0];
  let dj = vec![0, 1, 0, -1];

  let mut i: i32 = stp_i;
  let mut j: i32 = stp_j;
  let mut dir: usize = 0;
  let mut done_tiles: HashSet<(i32, i32)> = HashSet::new();
  done_tiles.insert((i, j));

  while true {
    let ii = i + di[dir];
    let jj = j + dj[dir];
    if ii < 0 || ii >= n as i32 || jj < 0 || jj >= m as i32 || arr[ii as usize].chars().nth(jj as usize).unwrap() == ' ' {
      break;
    }
    if arr[ii as usize].chars().nth(jj as usize).unwrap() == '#' {
      dir = (dir + 1) % 4;
      continue;
    }
    i = ii;
    j = jj;
    done_tiles.insert((i, j));
  }

  println!("PART 1: {}", done_tiles.len());
}

fn work_part_2(stp_i: i32, stp_j: i32, arr: Vec<String>) -> u32 {
  let n = arr.len();
  let m = arr[0].len();
  let mut done_tiles: HashSet<(i32, i32, u32)> = HashSet::new();

  let di = vec![-1, 0, 1, 0];
  let dj = vec![0, 1, 0, -1];
  let mut i: i32 = stp_i;
  let mut j: i32 = stp_j;
  let mut dir: u32 = 0;
  done_tiles.insert((stp_i, stp_j, dir));

  while true {
    let ii = i + di[dir as usize];
    let jj = j + dj[dir as usize];
    if ii < 0 || ii >= n as i32 || jj < 0 || jj >= m as i32 || arr[ii as usize].chars().nth(jj as usize).unwrap() == ' ' {
      break;
    }
    if arr[ii as usize].chars().nth(jj as usize).unwrap() == '#' {
      dir = (dir + 1) % 4;
      continue;
    }
    i = ii;
    j = jj;
    if done_tiles.contains(&(i, j, dir)) {
      return 1;
    }
    done_tiles.insert((i, j, dir));
  }
  return 0;
}

fn part_2(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  
  let arr = parse_input(input_file_name);
  let n = arr.len();
  let m = arr[0].len();

  let mut stp_i: i32 = -1;
  let mut stp_j: i32 = -1;
  for i in 0..n {
    for j in 0..m {
      if arr[i].chars().nth(j).unwrap() == '^' {
        stp_i = i as i32;
        stp_j = j as i32;
        break;
      }
    }
  }

  let mut count: u32 = 0;
  for i in 0..n {
    for j in 0..m {
      if arr[i].chars().nth(j).unwrap() == '#' || arr[i].chars().nth(j).unwrap() == '^' {
        continue;
      }
      let mut tmp_arr  = arr.clone();
      tmp_arr[i as usize].replace_range(j..j+1, "#");
      count += work_part_2(stp_i, stp_j, tmp_arr);
    }
  }

  println!("PART 2: {}", count);
}

fn main() {
  let input_file_name = std::env::args().nth(1).unwrap();
  
  part_1(&input_file_name);
  println!("====================");
  part_2(&input_file_name);
}