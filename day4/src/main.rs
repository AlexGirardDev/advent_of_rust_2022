const INPUT: &str = include_str!("./input.txt");
const TEST: &str = include_str!("./test.txt");

fn main() {
    //part_1();
        part_2();
}

fn part_1() {
    let mut count = 0;
    for (i, line) in INPUT.lines().enumerate() {
        //36-92,35-78
        let split: Vec<&str> = line.split(",").collect();
        if split.len() != 2 {
            panic!("reee");
        }
        let range_1_split: Vec<&str> = split[0].split("-").collect();
        if range_1_split.len() != 2 {
            panic!("reee");
        }

        let range_2_split: Vec<&str> = split[1].split("-").collect();
        if range_2_split.len() != 2 {
            panic!("reee");
        }
        let num1_lower = range_1_split[0].parse::<i128>().unwrap();
        let num1_upper = range_1_split[1].parse::<i128>().unwrap();

        let num2_lower = range_2_split[0].parse::<i128>().unwrap();
        let num2_upper = range_2_split[1].parse::<i128>().unwrap();

        let mut num_1_mask: i128 = 0;
        let mut num_2_mask: i128 = 0;

        for i in num1_lower..num1_upper+1 {
            num_1_mask |= 1 << i;
        }

        for i in num2_lower..num2_upper+1 {
            num_2_mask |= 1 << i;
        }
        let result = num_1_mask & num_2_mask;

        if result == num_1_mask || result == num_2_mask {
            //println!( "YES {} {}", i, line);
            count += 1;
        }
        else
        {
            //println!( "NO {} {}", i, line);
        }
    }
    println!("day 4_1 {}", count);
}

fn part_2() {
    let mut count = 0;
    for (i, line) in INPUT.lines().enumerate() {
        //36-92,35-78
        let split: Vec<&str> = line.split(",").collect();
        if split.len() != 2 {
            panic!("reee");
        }
        let range_1_split: Vec<&str> = split[0].split("-").collect();
        if range_1_split.len() != 2 {
            panic!("reee");
        }

        let range_2_split: Vec<&str> = split[1].split("-").collect();
        if range_2_split.len() != 2 {
            panic!("reee");
        }
        let num1_lower = range_1_split[0].parse::<i128>().unwrap();
        let num1_upper = range_1_split[1].parse::<i128>().unwrap();

        let num2_lower = range_2_split[0].parse::<i128>().unwrap();
        let num2_upper = range_2_split[1].parse::<i128>().unwrap();

        let mut num_1_mask: i128 = 0;
        let mut num_2_mask: i128 = 0;

        for i in num1_lower..num1_upper+1 {
            num_1_mask |= 1 << i;
        }

        for i in num2_lower..num2_upper+1 {
            num_2_mask |= 1 << i;
        }
        let result = num_1_mask & num_2_mask;

        if result != 0{
            //println!( "YES {} {}", i, line);
            count += 1;
        }
        else
        {
            //println!( "NO {} {}", i, line);
        }
    }
    println!("day 4_1 {}", count);
}
