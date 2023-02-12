use std::collections::{HashMap};

const INPUT: &str = include_str!("input.txt");
//const INPUT: &str = include_str!("./test.txt");
//const TEST: &str = include_str!("./test2.txt");

fn main() {
    part_2();
}

fn part_1() {
    let mut line_iter = INPUT.lines();
    let mut path_stack: Vec<&str> = Vec::new();
    let mut dir_hashmap: HashMap<String, i32> = HashMap::new();
    loop
    {
        let line = match line_iter.next() {
            None => { break; }
            Some(l) => { l }
        };
        println!("{line}");

        match line {
            s if s.starts_with("$ cd ..") => {
                match path_stack.pop() {
                    None => {
                        println!("???")                        ;
                    }
                    Some(_) => {}
                };
            }
            s if s.starts_with("$ cd /") => {
                path_stack.clear();
            }
            s if s.starts_with("$ cd") => {
                path_stack.push(&s[5..]);
            }
            s if s.starts_with("$ ls") => {}
            s if s.starts_with("dir ") => {
            }
            s => {
                let size: (&str, i32) = match s.split_once(" ") {
                    None => {
                        println!("???");
                        ("", 0)
                    }
                    Some(x) => {
                        let parse: i32 = match x.0.parse() {
                            Err(e) => {
                                println!("{} ,{}", x.0, e);
                                0
                            }
                            Ok(x) => { x }
                        };
                        (x.1, parse)
                    }
                };

                println!("{:?}",path_stack);
                if path_stack.len()>0 {
                    for i in 0..path_stack.len()
                    {
                        let dir = path_stack[..i+1].join("/");
                        println!("dir = {dir} {i} ");

                        let size = size.1 + if dir_hashmap.contains_key(dir.as_str()) { dir_hashmap[dir.as_str()] } else { 0 };
                        dir_hashmap.insert(dir, size);
                    }

                }
            }
        }
    }
    let result: i32 = dir_hashmap.iter().filter(|x1| { *x1.1 <= 100000 }).map(|x2| { x2.1 }).sum();
    println!("{result}");
}

fn part_2() {
    let mut line_iter = INPUT.lines();
    let mut path_stack: Vec<&str> = Vec::new();
    let mut dir_hashmap: HashMap<String, i32> = HashMap::new();
    dir_hashmap.insert(String::from("/"),0);
    loop
    {
        let line = match line_iter.next() {
            None => { break; }
            Some(l) => { l }
        };
        println!("{line}");

        match line {
            s if s.starts_with("$ cd ..") => {
                match path_stack.pop() {
                    None => {
                        println!("???")                        ;
                    }
                    Some(_) => {}
                };
            }
            s if s.starts_with("$ cd /") => {
                path_stack.clear();
            }
            s if s.starts_with("$ cd") => {
                path_stack.push(&s[5..]);
            }
            s if s.starts_with("$ ls") => {}
            s if s.starts_with("dir ") => {
            }
            s => {
                let size: (&str, i32) = match s.split_once(" ") {
                    None => {
                        println!("???");
                        ("", 0)
                    }
                    Some(x) => {
                        let parse: i32 = match x.0.parse() {
                            Err(e) => {
                                println!("{} ,{}", x.0, e);
                                0
                            }
                            Ok(x) => { x }
                        };
                        (x.1, parse)
                    }
                };


                println!("{:?}",path_stack);
                dir_hashmap.insert(String::from("/"), size.1 + dir_hashmap["/"]);
                if path_stack.len()>0 {
                    for i in 0..path_stack.len()
                    {
                        let dir = path_stack[..i+1].join("/");
                        println!("dir = {dir} {i} ");

                        let size = size.1 + if dir_hashmap.contains_key(dir.as_str()) { dir_hashmap[dir.as_str()] } else { 0 };
                        dir_hashmap.insert(dir, size);
                    }
                }
            }
        }
    }
    let total = dir_hashmap.get("/").unwrap();

    println!("total {total}");
    let space_needed =30_000_000- (70_000_000 - total) ;
    println!("spaceNeeded {space_needed}");

    let result: &i32 = dir_hashmap.iter().filter(|x1| { *x1.1 >= space_needed }).map(|x2| {x2.1}).min().unwrap();
    println!("{result}");
}
//fn part_2() {}

