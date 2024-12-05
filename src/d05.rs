use std::collections::HashSet;
use std::io::BufRead;

fn parse_input(input_file_name: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
  let input = std::fs::File::open(input_file_name).unwrap();

  let mut rules: HashSet<(u32, u32)> = HashSet::new();
  let mut updates: Vec<Vec<u32>> = Vec::new();
  let mut state = 0;
  for line in std::io::BufReader::new(input).lines() {
    let cur_line = line.unwrap();
    if cur_line.is_empty() {
      state = 1;
    }
    else if state == 0 {
      cur_line
        .split_once('|')
        .map(|(a, b)| {
          let a = a.trim().parse::<u32>().unwrap();
          let b = b.trim().parse::<u32>().unwrap();
          rules.insert((a, b));
        });
    }
    else {  
      let mut cur_updates: Vec<u32> = Vec::new();
      for cur_num in cur_line.split(',') {
        cur_updates.push(cur_num.trim().parse::<u32>().unwrap());
      }
      updates.push(cur_updates);
    }
  }

  return (rules, updates);
}

fn part_1(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  
  let (rules, updates) = parse_input(input_file_name);
 
  let sum: u32 = updates
    .into_iter()
    .filter_map(|update| {
      update
        .is_sorted_by(|a, b| !rules.contains(&(*b, *a)))
        .then_some(u32::from(update[update.len() / 2]))
    })
    .sum();

  println!("PART 1: {}", sum);
}

fn part_2(input_file_name: &str) {
  println!("INPUT: {}", input_file_name);
  
  let (rules, updates) = parse_input(input_file_name);

  let sum: u32 = updates
    .into_iter()
    .filter_map(|mut update| {
      (!update.is_sorted_by(|a, b| !rules.contains(&(*b, *a))))
        .then_some({
          update.sort_by(|a, b| {
            if rules.contains(&(*a, *b)) {
              std::cmp::Ordering::Less
            }
            else {
              std::cmp::Ordering::Greater
            }
          });
          u32::from(update[update.len() / 2])
        })
    })
    .sum();

  println!("PART 2: {}", sum);
}

fn main() {
  let input_file_name = std::env::args().nth(1).unwrap();
  
  part_1(&input_file_name);
  println!("====================");
  part_2(&input_file_name);
}