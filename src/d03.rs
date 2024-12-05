use std::io::BufRead;

fn part_1(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let input = std::fs::File::open(input_file_name).unwrap();

  let mut arr: Vec<String> = Vec::new();
  for line in std::io::BufReader::new(input).lines() {
    let cur_line = line.unwrap();
    arr.push(cur_line);
  }

  let mut sum: u32 = 0;
  let mut word = String::new();
  arr.into_iter().for_each(|line| {
    line.chars().for_each(|c| match c {
      'm' => {
        word.push(c);
      }
      'u' if word == "m" => {
        word.push(c);
      }
      'l' if word == "mu" => {
        word.push(c);
      }
      '(' if word == "mul" => {
        word.push(c);
      }
      d if d.is_ascii_digit() && word.starts_with("mul(") && word.chars().skip(4).all(|c| c.is_ascii_digit() || c == ',') => {
        word.push(d);
      }
      ',' if word.starts_with("mul(") && word.chars().skip(4).all(|c| c.is_ascii_digit()) => {
        word.push(c);
      }
      ')' if word.starts_with("mul(") => {
        if let Some((a, b)) = word[4..].split_once(',') {
          if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
            sum += a * b;
          }
        }
        word.clear();
      }
      _ => {
        word.clear();
      }
    });
  });

  println!("PART 1: {}", sum);
}

fn part_2(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  let input = std::fs::File::open(input_file_name).unwrap();

  let mut arr: Vec<String> = Vec::new();
  for line in std::io::BufReader::new(input).lines() {
    let cur_line = line.unwrap();
    arr.push(cur_line);
  }

  let mut sum: u32 = 0;
  let mut word = String::new();
  let mut enabled = true;
  arr.into_iter().for_each(|line| {
    line.chars().for_each(|c| match c {
      'm' => {
        word.push(c);
      }
      'u' if word == "m" => {
        word.push(c);
      }
      'l' if word == "mu" => {
        word.push(c);
      }
      'd' => {
        word.push(c);
      }
      'o' if word == "d" => {
        word.push(c);
      }
      'n' if word == "do" => {
        word.push(c);
      }
      '\'' if word == "don" => {
        word.push(c);
      }
      't' if word == "don'" => {
        word.push(c);
      }
      '(' if (enabled && (word == "mul" || word == "don't")) || word == "do" => {
        word.push(c);
      }
      d if d.is_ascii_digit() && word.starts_with("mul(") && word.chars().skip(4).all(|c| c.is_ascii_digit() || c == ',') => {
        word.push(d);
      }
      ',' if word.starts_with("mul(") && word.chars().skip(4).all(|c| c.is_ascii_digit()) => {
        word.push(c);
      }
      ')' => {
        if enabled {
          if let Some(word) = word.strip_prefix("mul(") {
            if let Some((a, b)) = word.split_once(',') {
              if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                sum += a * b;
              }
            }
          }
          else if word == "don't(" {
            enabled = false;
          }
        }
        else if word == "do(" {
          enabled = true;
        }
        word.clear();
      }
      _ => {
        word.clear();
      }
    });
  });

  println!("PART 2: {}", sum);
}

fn main() {
  let input_file_name = std::env::args().nth(1).unwrap();
  
  part_1(&input_file_name);
  println!("====================");
  part_2(&input_file_name);
}