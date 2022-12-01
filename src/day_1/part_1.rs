
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn partone() {
  let file_result = File::open("./data/day_1.txt");
  let file = match file_result {
      Ok(file) => file,
      Err(error) => panic!("should not error")
  };

  let reader = BufReader::new(file);

  let mut most_calories: i32 = 0;
  let mut current_calories: i32 = 0;

  // go through elf calories
  for line in reader.lines() {
      let line_contents = line.unwrap();
      if line_contents.is_empty() {
          // set most calories if it is, and reset current calories
          if current_calories > most_calories {
              most_calories = current_calories;
          }
          current_calories = 0;
      } else {
          let calorie: i32 = line_contents.parse().unwrap();
          current_calories += calorie;
      }
  }
  
  if current_calories > most_calories {
      most_calories = current_calories;
  }

  println!("Most calories: {}", most_calories);
}