use std::io::BufRead;

fn part_1(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let input = std::fs::File::open(input_file_name).unwrap();

  let mut arr: Vec<String> = Vec::new();
  for line in std::io::BufReader::new(input).lines() {
    let cur_line = line.unwrap();
    arr.push(cur_line);
  }

  let di = vec![-1, -1, -1, 0, 0, 1, 1, 1];
  let dj = vec![-1, 0, 1, -1, 1, -1, 0, 1];
  let xmas = vec!['X', 'M', 'A', 'S'];

  let mut count: u32 = 0;
  let n: i32 = arr.len() as i32;
  let m: i32 = arr[0].len() as i32;
  for i in 0..n {
    for j in 0..m {
      if arr[i as usize].chars().nth(j as usize).unwrap() != 'X' {
        continue;
      }
      for k in 0..8 {
        let mut ii: i32 = i as i32;
        let mut jj: i32 = j as i32;
        let mut ok = true;
        for l in 1..4 {
          ii += di[k];
          jj += dj[k];
          if ii < 0 || ii >= n || jj < 0 || jj >= m {
            ok = false;
            break;
          }
          if arr[ii as usize].chars().nth(jj as usize).unwrap() != xmas[l] {
            ok = false;
            break;
          }
        }
        if ok {
          count += 1;
        }
      }
    }
  }

  println!("PART 1: {}", count);
}

fn part_2(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let input = std::fs::File::open(input_file_name).unwrap();

  let mut arr: Vec<String> = Vec::new();
  for line in std::io::BufReader::new(input).lines() {
    let cur_line = line.unwrap();
    arr.push(cur_line);
  }

  let mut count: u32 = 0;
  let n: i32 = arr.len() as i32;
  let m: i32 = arr[0].len() as i32;
  for i in 1..(n-1) {
    for j in 1..(m-1) {
      if arr[i as usize].chars().nth(j as usize).unwrap() != 'A' {
        continue;
      }
      if arr[i as usize - 1].chars().nth(j as usize - 1).unwrap() == 'M' && arr[i as usize - 1].chars().nth(j as usize + 1).unwrap() == 'S' 
      && arr[i as usize + 1].chars().nth(j as usize - 1).unwrap() == 'M' && arr[i as usize + 1].chars().nth(j as usize + 1).unwrap() == 'S' {
        count += 1;
      }
      if arr[i as usize - 1].chars().nth(j as usize - 1).unwrap() == 'M' && arr[i as usize - 1].chars().nth(j as usize + 1).unwrap() == 'M' 
      && arr[i as usize + 1].chars().nth(j as usize - 1).unwrap() == 'S' && arr[i as usize + 1].chars().nth(j as usize + 1).unwrap() == 'S' {
        count += 1;
      }
      if arr[i as usize - 1].chars().nth(j as usize - 1).unwrap() == 'S' && arr[i as usize - 1].chars().nth(j as usize + 1).unwrap() == 'M' 
      && arr[i as usize + 1].chars().nth(j as usize - 1).unwrap() == 'S' && arr[i as usize + 1].chars().nth(j as usize + 1).unwrap() == 'M' {
        count += 1;
      }
      if arr[i as usize - 1].chars().nth(j as usize - 1).unwrap() == 'S' && arr[i as usize - 1].chars().nth(j as usize + 1).unwrap() == 'S' 
      && arr[i as usize + 1].chars().nth(j as usize - 1).unwrap() == 'M' && arr[i as usize + 1].chars().nth(j as usize + 1).unwrap() == 'M' {
        count += 1;
      }
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