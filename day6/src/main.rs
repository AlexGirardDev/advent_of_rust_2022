use std::collections::HashSet;
use std::io::Lines;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = include_str!("./test.txt");

fn main() {
    //part_1();
    part_2();
}

fn part_1() {
    println!("{}", TEST);
    for (i, c) in INPUT.chars().enumerate()
    {
        println!("{}", i);
        if i < 4
        {
            continue;
        }
        let sub_slice: HashSet<char> = INPUT[i - 4..i].chars().collect();

        println!("woweeee {:?} - {}", sub_slice, i);
        if sub_slice.len() == 4
        {
            println!("woweeee {} - {}", i, c);
            return;
        }
    }
}

fn part_2() {
    println!("{}", TEST);
    let distinctLen = 14;
    for (i, c) in INPUT.chars().enumerate()
    {
        println!("{}", i);
        if i < distinctLen
        {
            continue;
        }
        let sub_slice: HashSet<char> = INPUT[i - distinctLen..i].chars().collect();

        println!("woweeee {:?} - {}", sub_slice, i);
        if sub_slice.len() == distinctLen
        {
            println!("woweeee {} - {}", i, c);
            return;
        }
    }
}
