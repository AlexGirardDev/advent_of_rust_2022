use std::borrow::Borrow;
use colored::Colorize;
use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const TEST: &str = include_str!("test.txt");

fn main() {
    part_2();
}

fn part_2() {
    const RADIX: u32 = 10;

    let mut map: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| {
                    (
                        match c.to_digit(RADIX) {
                            None => 0,
                            Some(x) => x,
                        } as u8
                    )
                })
                .collect()
        })
        .collect();

    let max_x = map.len();
    let max_y = map.first().unwrap().len();


    let mut max_score = 0;
    for x in 1..max_x {
        for y in 1..max_y {
            let map_item = map[x][y];
            println!("{x},{y} !!!");
            print_grid_3(&map, (x,y));
            let mut score_vec: Vec<i32> = Vec::new();
            {
                let mut temp_score = 0;
                'inner: for x2 in x+1..max_x {
                    println!("{x2},{y} >");
                    let height = map[x2][y];
                    temp_score += 1;
                    if height >= map_item {
                        break 'inner;
                    }
                }

                score_vec.push(temp_score);
            }
            {
                let mut temp_score = 0;
                'inner: for x2 in (0..x).rev() {
                    println!("{x2},{y} <");
                    let height = map[x2][y];

                    temp_score += 1;
                    if height >= map_item {
                        break 'inner;
                    }

                }
                score_vec.push(temp_score);
            }
            {
                let mut temp_score = 0;
                'inner: for y2 in y..max_y {
                    println!("{x},{y2} v");
                    let height = map[x][y2];
                    temp_score += 1;
                    if height >= map_item {
                        break 'inner;
                    }

                    // if height >= tallest_tree {
                    //     temp_score += 1;
                    //     tallest_tree = height;
                    // }
                }
                score_vec.push(temp_score);
            }
            {
                let mut temp_score = 0;
                'inner: for y2 in (0..y).rev() {
                    println!("{x},{y2} ^");
                    let height = map[x][y2];

                    temp_score += 1;
                    if height > map_item {
                        println!("oof {x},{y2} - {map_item} - {height}");
                        break 'inner;
                    }
                }
                score_vec.push(temp_score);
            }
            let mut new_score = *score_vec.first().unwrap();
            for s in score_vec.iter().skip(1)
            {
                    new_score = new_score * s;
            }
            if new_score > max_score {
                max_score = new_score;
            }

            if new_score>0{
                println!("{x},{y} - {}- {new_score} {:?}",map_item, score_vec);

            }
        }
    }
    println!("{max_score}");
}
fn part_1() {
    const RADIX: u32 = 10;

    let mut map: Vec<Vec<(u8, bool)>> = INPUT
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| {
                    (
                        match c.to_digit(RADIX) {
                            None => 0,
                            Some(x) => x,
                        } as u8,
                        true,
                    )
                })
                .collect()
        })
        .collect();

    let max_x = map.len();
    let max_y = map.first().unwrap().len();

    for x in 0..max_x {
        let mut map_item = map[x][0];
        map_item.1 = false;
        map[x][0] = map_item;

        let mut map_item = map[x][max_y - 1];
        map_item.1 = false;
        map[x][max_y - 1] = map_item;
    }

    for y in 1..max_y - 1 {
        let mut map_item = map[0][y];
        map_item.1 = false;
        map[0][y] = map_item;

        let mut map_item = map[max_x - 1][y];
        map_item.1 = false;
        map[max_x - 1][y] = map_item;
    }

    for x in 1..max_x - 1 {
        let mut tallest_tree: u8 = 0;
        let mut tallest_tree_inverse: u8 = 0;
        for y in 0..max_y - 1 {
            let mut map_item = map[x][y];
            // println!("{x},{y} - {tallest_tree} - {} ", map_item.0);
            // print_grid(&map,(x,y) );
            if map_item.0 > tallest_tree {
                map_item.1 = false;
                map[x][y] = map_item;
                tallest_tree = map_item.0;
            }

            let y = max_y - 1 - y;
            let mut map_item = map[x][y];
            //println!("{x},{y} - {tallest_tree_inverse} - {} INVS ", map_item.0);
            //print_grid(&map,(x,y) );
            if map_item.0 > tallest_tree_inverse {
                map_item.1 = false;
                map[x][y] = map_item;
                tallest_tree_inverse = map_item.0;
            }
        }
    }
    //print_grid(&map);
    for y in 1..max_y - 1 {
        let mut tallest_tree: u8 = 0;
        let mut tallest_tree_inverse: u8 = 0;
        for x in 0..max_x - 1 {
            let mut map_item = map[x][y];
            // println!("{x},{y} - {tallest_tree} - {} ", map_item.0);
            // print_grid(&map,(x,y) );
            if map_item.0 > tallest_tree {
                map_item.1 = false;
                map[x][y] = map_item;
                tallest_tree = map_item.0;
            }

            let x = max_x - 1 - x;
            let mut map_item = map[x][y];
            if map_item.0 > tallest_tree_inverse {
                if map_item.1 {
                    println!("{x},{y} - {tallest_tree_inverse} - {} INVS ", map_item.0);
                    print_grid(&map, (x, y));
                }
                map_item.1 = false;
                map[x][y] = map_item;
                tallest_tree_inverse = map_item.0;
            }
        }
    }
    print_grid(&map, (1000, 1000));
    let result: usize = map
        .iter()
        .map(|x1| x1.iter().filter(|x2| !x2.1).count())
        .sum();
    println!("result = {result}");
}
fn print_grid(map: &Vec<Vec<(u8, bool)>>, current_tree: (usize, usize)) {
    for (x, line) in map.iter().enumerate() {
        for (y, s) in line.iter().enumerate() {
            if current_tree == (x, y) {
                print!("{}", s.0.to_string().bright_yellow())
            } else if s.1 {
                print!("{}", s.0.to_string().green())
            }
        }

        println!("");
    }
}
fn print_grid_2(map: &Vec<Vec<u8>>, current_tree: (usize, usize)) {
    for (x, line) in map.iter().enumerate() {
        for (y, s) in line.iter().enumerate() {
            if current_tree == (x, y) {
                print!("{}", s.to_string().bright_yellow())
            } else {
                print!("{}", s.to_string().green())
            }
        }

        println!("");
    }
}

fn print_grid_3(map: &Vec<Vec<u8>>, current_tree: (usize, usize)) {
    for (x, line) in map.iter().skip(current_tree.0-2).take(5).enumerate() {
        for (y, s) in line.iter().skip(current_tree.1-2).take(5).enumerate() {
            if current_tree == (x, y) {
                print!("{}", s.to_string().bright_yellow())
            } else {
                print!("{}", s.to_string().green())
            }
        }

        println!("");
    }
}
