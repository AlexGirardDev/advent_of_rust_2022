const INPUT: &str = include_str!("./input.txt");
const TEST: &str = include_str!("./test.txt");

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut score: i32 = 0;

    for line in INPUT.lines() {
        let len = line.len();
        let mut binary: u64 = 0;
        for (i, c) in line.chars().enumerate() {
            let c_code = c as u8;

            let index = match c_code {
                65..=90 => c_code - 38,  // upercase codes are 65-90
                97..=122 => c_code - 96, //lower case codes are 97-122
                _ => 0,
            };
            //println!("{} - {} - {} ", i, c, index);

            if i < len / 2 {
                binary |= 1 << index;
            } else {
                if (binary >> index) & 1 == 1 {
                    score = score + i32::from(index);
                    //println!("bad char {}", c);

                    break;
                }
            }
        }
    }
    println!("day3_1 {}", score);
}

fn part_2() {
    let map: Vec<u64> = INPUT
        .lines()
        .map(|line| {
            let mut binary: u64 = 0;
            for (_, c) in line.chars().enumerate() {
                let c_code = c as u8;
                let index = match c_code {
                    65..=90 => c_code - 38,  // upercase codes are 65-90
                    97..=122 => c_code - 96, //lower case codes are 97-122
                    _ => 0,
                };
                binary |= 1 << index;
            }
            //println!("{:053b}", binary);
            return binary;
        })
        .collect();

    let mut score = 0;
    let mut i = 1;
    while i < map.len() {
        let result = map[i - 1] & map[i] & map[i + 1];
            println!("{} ", result);
        if result != 0 {

            //score += log_result as i32;
            break;
        }
        i += 3;
    }
    println!("day3_2 {}", score);
}

fn print_key_codes() {
    let mut i: u8 = 0;
    while i < 182 {
        println!(" {} - {} ", i, i as char);
        i += 1;
    }
}
