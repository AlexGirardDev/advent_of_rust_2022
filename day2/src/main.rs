use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = include_str!("./input.txt");


fn part_1() {
    let mut score: i32 = 0;

    for line in INPUT.lines() {
        let your_hand_str = &line[..1];
        let there_hand_str = &line[2..];
        let there_hand_val = match your_hand_str {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("woops"),
        };
        let plan = Plan {
            your_hand: match there_hand_str {
                "X" => match there_hand_val {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper,
                },
                "Y" => match there_hand_val {
                    Hand::Rock => Hand::Rock,
                    Hand::Paper => Hand::Paper,
                    Hand::Scissors => Hand::Scissors,
                },

                "Z" => match there_hand_val {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock,
                },
                _ => panic!("woops"),
            },
            there_hand: there_hand_val,
        };

        score += match plan.your_hand {
            Hand::Rock => {
                let bonus = 1;
                match plan.there_hand {
                    Hand::Rock => 3 + bonus,
                    Hand::Paper => 0 + bonus,
                    Hand::Scissors => 6 + bonus,
                }
            }
            Hand::Paper => {
                let bonus = 2;
                match plan.there_hand {
                    Hand::Rock => 6 + bonus,
                    Hand::Paper => 3 + bonus,
                    Hand::Scissors => 0 + bonus,
                }
            }
            Hand::Scissors => {
                let bonus = 3;
                match plan.there_hand {
                    Hand::Rock => 0 + bonus,
                    Hand::Paper => 6 + bonus,
                    Hand::Scissors => 3 + bonus,
                }
            }
        };
    }
    println!("day2_2 {}", score);
}
 fn part_2() {
    let mut score: i32 = 0;

    for line in INPUT.lines() {
        let your_hand_str = &line[..1];
        let there_hand_str = &line[2..];
        let plan = Plan {
            there_hand: match your_hand_str {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissors,
                _ => panic!("woops"),
            },

            your_hand: match there_hand_str {
                "X" => Hand::Rock,
                "Y" => Hand::Paper,
                "Z" => Hand::Scissors,
                _ => panic!("woops"),
            },
        };

        score += match plan.your_hand {
            Hand::Rock => {
                let bonus = 1;
                match plan.there_hand {
                    Hand::Rock => 3 + bonus,
                    Hand::Paper => 0 + bonus,
                    Hand::Scissors => 6 + bonus,
                }
            }
            Hand::Paper => {
                let bonus = 2;
                match plan.there_hand {
                    Hand::Rock => 6 + bonus,
                    Hand::Paper => 3 + bonus,
                    Hand::Scissors => 0 + bonus,
                }
            }
            Hand::Scissors => {
                let bonus = 3;
                match plan.there_hand {
                    Hand::Rock => 0 + bonus,
                    Hand::Paper => 6 + bonus,
                    Hand::Scissors => 3 + bonus,
                }
            }
        };
    }
    println!("day2_1 {}", score);
}

#[derive(Debug)]
struct Plan {
    your_hand: Hand,
    there_hand: Hand,
}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}


fn main() {
    part_1();
    part_2();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
