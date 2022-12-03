use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn part_1() {
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
        let my_pick = vec[1].chars().next().expect("string is empty");

        let mypick_amount = match my_pick {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
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
            'X' => return 3, // opponent rock you rock draw
            'Y' => return 6, // opponent rock you paper you win
            'Z' => return 0, // opponent rock you scissor you lose
            _ => return 0
        }
    } else if opponent == 'B' {
        match user {
            'X' => return 0, // opponent paper you rock you lose
            'Y' => return 3, // opponent ppaer you paper youy draw
            'Z' => return 6, // opponent paper you scissor you win
            _ => return 0
        }
    } else if opponent == 'C' {
        match user {
            'X' => return 6, // opponent scissor you rock u win
            'Y' => return 0, // opponent scissor you scissor u lose
            'Z' => return 3, // opponent scissor you scicor you draw
            _ => return 0
        }
    } else {
        panic!("NOT HERE PLS");
    }
}