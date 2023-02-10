use std::collections::{HashMap};

const INPUT: &str = include_str!("input.txt");
const TEST: &str = include_str!("./test.txt");

fn main() {
    part_1();
}

fn part_1() {
    let mut line_iter = INPUT.lines();
    let mut path = String::new();
    let mut tld_hashmap: HashMap<&str, u32> = HashMap::new();
    let mut current_dir = "";
    loop
    {
        let line = match line_iter.next() {
            None => { return; }
            Some(l) => { l }
        };


        match line {
            s if s.starts_with("$ cd ..") => {
                let last_slash = path.rfind("/").unwrap();
                path = (&path[..last_slash]).to_string();
            }
            s if s.starts_with("$ cd /") => { path = String::new() }
            s if s.starts_with("$ cd") => {
                path = format!("{}/{}", &path.as_str(), &s[5..]);
            }
            s if s.starts_with("$ ls") => {}
            s => {
                let mut size: u32 = match s.split_once(" ") {
                    None => { 0 }
                    Some(x) => {
                        x.0.parse().unwrap()
                    }
                };
                match path.rfind("/") {
                    None => {

                    }
                    Some(i) => {
                        // let dir = &path[i..];
                        // tld_hashmap.insert(&dir, size + if tld_hashmap.contains_key(&dir) { tld_hashmap[&dir] } else { 0 }).unwrap();
                    }
                };
            }
        }
        println!("{:?}",tld_hashmap)
    }
}

fn proccess_next() {}

fn part_2() {}

