use std::fs::File;
use std::io::{prelude::*, BufReader};


// A = Rock
// B = paper
// C = scissor
#[allow(dead_code)]
pub fn part_2() {
    let file_result = File::open("./data/day_2.txt");
    let file = file_result.unwrap();

    let reader = BufReader::new(file);
    let mut scores: Vec<u32> = Vec::new();

    // go through elf calories
    for linebuf in reader.lines() {
        let line = linebuf.unwrap();
        let split = line.split(' ');
        let vec = split.collect::<Vec<&str>>();
        
        let opponent_pick = vec[0].chars().next().expect("string is empty");
        let expected_result = vec[1].chars().next().expect("string is empty");

        let mut my_pick: char = 'H';

        if expected_result == 'X' {
          my_pick = match opponent_pick {
            'A' => 'C',
            'B' => 'A',
            'C' => 'B',
            _ => panic!("NO")
          };
        } else if expected_result == 'Y' {
          my_pick = match opponent_pick {
            'A' => 'A',
            'B' => 'B',
            'C' => 'C',
            _ => panic!("NO")
          };
        } else if expected_result == 'Z' {
          my_pick = match opponent_pick {
            'A' => 'B',
            'B' => 'C',
            'C' => 'A',
            _ => panic!("NO")
          };
        }

        let mypick_amount = match my_pick {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0
        };

        let score = mypick_amount + get_result(opponent_pick, my_pick);
        scores.push(score);
    }

    let sum = scores.into_iter().reduce(|a, b| a + b).unwrap();
    println!("Score sum: {}", sum);
}


fn get_result(opponent: char, user: char) -> u32 {
    if opponent == 'A' {
        match user {
            'A' => return 3, // opponent rock you rock draw
            'B' => return 6, // opponent rock you paper you win
            'C' => return 0, // opponent rock you scissor you lose
            _ => return 0
        }
    } else if opponent == 'B' {
        match user {
            'A' => return 0, // opponent paper you rock you lose
            'B' => return 3, // opponent ppaer you paper youy draw
            'C' => return 6, // opponent paper you scissor you win
            _ => return 0
        }
    } else if opponent == 'C' {
        match user {
            'A' => return 6, // opponent scissor you rock u win
            'B' => return 0, // opponent scissor you scissor u lose
            'C' => return 3, // opponent scissor you scicor you draw
            _ => return 0
        }
    } else {
        panic!("NOT HERE PLS");
    }
}
