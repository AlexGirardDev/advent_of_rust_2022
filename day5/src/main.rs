const INPUT: &str = include_str!("./input.txt");
const TEST: &str = include_str!("./test.txt");

fn main() {
    //part_1();
    part_2();
}

fn part_1() {
    let lines: Vec<&str> = INPUT.split("\n").collect();
    let stack_input_index = lines.iter().position(|&r| r == "").unwrap();
    let stack_input = &lines[..stack_input_index];
    let move_input = &lines[stack_input_index + 1..];
    let mut stack_list: [Vec<char>; 3] = Default::default();
    for x in 0..10 {
        for y in (0..stack_input_index - 1).rev() {
            let char = stack_input[y].chars().nth(1 + (x * 4));
            match char {
                Some(char) => {
                    if char != ' ' {
                        stack_list[x].push(char)
                    }
                }
                None => (),
            }
        }
    }
    println!("{:?} ", stack_list);
    println!("{:?} ", move_input[0]);

    for line in move_input {
        println!("{:?} ", stack_list);
        if line.len() < 10 {
            break;
        }
        let from_split = line[5..].split_once(" from ").unwrap();
        let to_split = from_split.1.split_once(" to ").unwrap();

        let items = from_split.0.parse::<usize>().unwrap();
        let from = to_split.0.parse::<usize>().unwrap() - 1;
        let to = to_split.1.parse::<usize>().unwrap() - 1;
        for _ in 0..items {
            let item = stack_list[from].pop();
            match item {
                Some(item) => stack_list[to].push(item),
                None => (),
            };
        }
    }
    println!("{:?} ", stack_list);

    let result: String = stack_list
        .iter()
        .map(|v| v.last().copied().unwrap())
        .collect();
    println!("{:?} ", result);
}

fn part_2() {
    let lines: Vec<&str> = INPUT.split("\n").collect();
    let stack_input_index = lines.iter().position(|&r| r == "").unwrap();
    let stack_input = &lines[..stack_input_index];
    let move_input = &lines[stack_input_index + 1..];
    let mut stack_list: [Vec<char>; 9] = Default::default();
    for x in 0..10 {
        for y in (0..stack_input_index - 1).rev() {
            let char = stack_input[y].chars().nth(1 + (x * 4));
            match char {
                Some(char) => {
                    if char != ' ' {
                        stack_list[x].push(char)
                    }
                }
                None => (),
            }
        }
    }
    println!("{:?} ", stack_list);
    println!("{:?} ", move_input[0]);

    for line in move_input {
        println!("{:?} ", stack_list);
        if line.len() < 10 {
            break;
        }
        let from_split = line[5..].split_once(" from ").unwrap();
        let to_split = from_split.1.split_once(" to ").unwrap();

        let items = from_split.0.parse::<usize>().unwrap();
        let from = to_split.0.parse::<usize>().unwrap() - 1;
        let to = to_split.1.parse::<usize>().unwrap() - 1;
        let stack: Vec<char> = stack_list[from]
            .drain(stack_list[from].len() - items ..)
            .collect();

        for i in stack {
            stack_list[to].push(i);
        }
    }
    println!("{:?} ", stack_list);

    let result: String = stack_list
        .iter()
        .map(|v| v.last().copied().unwrap())
        .collect();
    println!("{:?} ", result);
}
