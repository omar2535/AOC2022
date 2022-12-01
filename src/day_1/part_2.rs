
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn parttwo() {
  let file_result = File::open("./data/day_1.txt");
  let file = match file_result {
      Ok(file) => file,
      Err(error) => panic!("should not error")
  };

  let reader = BufReader::new(file);
  let mut largests: Vec<i32> = vec![0, 0, 0];

  let mut current_calories: i32 = 0;

  // go through elf calories
  for line in reader.lines() {
      let line_contents = line.unwrap();
      if line_contents.is_empty() {
        let min: i32 = *largests.iter().min().unwrap();
        if current_calories > min {
          let index = largests.iter().position(|&r| r == min).unwrap();
          largests[index] = current_calories;
        }
        current_calories = 0;
      } else {
          let calorie: i32 = line_contents.parse().unwrap();
          current_calories += calorie;
      }
  }
  
  let min: i32 = *largests.iter().min().unwrap();
  if current_calories > min {
    let index = largests.iter().position(|&r| r == min).unwrap();
    largests[index] = current_calories;
  }
  
  let result: i32 = largests.into_iter().reduce(|a, b| a+b).unwrap();

  println!("Total calories: {}", result);
}
