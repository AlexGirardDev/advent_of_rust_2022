use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");
//const TEST: &str = include_str!("./test.txt");


#[derive(Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
}

struct Movement {
    x: i32,
    y: i32,
}

fn main() {
    part_2();
}

fn part_2() {
    let mut head = Knot { x: 0, y: 0 };
    let mut knot_list: [Knot; 9] = [Knot { x: 0, y: 0 }; 9];
    let mut tal_positions: HashSet<(i32, i32)> = HashSet::new();

    for line in INPUT.lines() {
        // println!("{line}");
        let split = match line.split_once(" ") {
            None => { panic!("???") }
            Some(x) => { x }
        };

        let direction: Movement = match split.0 {
            "D" => Movement { x: 0, y: 1 },
            "U" => Movement { x: 0, y: -1 },
            "L" => Movement { x: -1, y: 0 },
            "R" => Movement { x: 1, y: 0 },
            _ => panic!("???")
        };

        let distance: i32 = split.1.parse().unwrap();
        for _ in 0..distance {
            head.x += direction.x;
            head.y += direction.y;
//            println!("head ({},{})", head.x, head.y);
            let mut prev_knot = &head;

            for mut knot in knot_list.iter_mut() {
                let diffs = (prev_knot.x - knot.x as i32, prev_knot.y - knot.y as i32);
//                println!("D:({},{}) K:({},{}) P:({},{})", diffs.0, diffs.1, knot.x, knot.y, prev_knot.x, prev_knot.y);

                if diffs.0.abs() + diffs.1.abs() > 2 {
                    knot.x += diffs.0.clamp(-1, 1);
                    knot.y += diffs.1.clamp(-1, 1);
                } else {
                    if diffs.0.abs() > 1 {
                        knot.x += diffs.0.clamp(-1, 1);
                    }
                    if diffs.1.abs() > 1 {
                        knot.y += diffs.1.clamp(-1, 1);
                    }
                }
                prev_knot = knot;
            }

            tal_positions.insert(knot_list.last().map(|x1|(x1.x, x1.y)).unwrap());

            //print_current_pos(&knot_list, head);
        }
    }
    println!("{}", &tal_positions.len());
    //println!("{:?}", &tal_positions);
}

// fn part_1() {
//     let mut h_x: i32 = 10;
//     let mut h_y: i32 = 10;
//     let mut t_x: i32 = 10;
//     let mut t_y: i32 = 10;
//
//     let mut tal_positions: HashSet<(i32, i32)> = HashSet::new();
//
//     for (i,line) in INPUT.lines().enumerate() {
//         let split = match line.split_once(" ") {
//             None => { panic!("???") }
//             Some(x) => { x }
//         };
//
//         let direction: (i32, i32) = match split.0 {
//             "D" => (0, 1),
//             "U" => (0, -1),
//             "L" => (-1, 0),
//             "R" => (1, 0),
//             _ => panic!("???")
//         };
//
//         let distance: i32 = split.1.parse().unwrap();
//         // println!("{line}");
//         for _ in 0..distance {
//             h_x += direction.0;
//             h_y += direction.1;
//             let x_diff = h_x.abs_diff(t_x);
//             let y_diff = h_y.abs_diff(t_y);
//             if x_diff + y_diff > 2
//             {
//                 //println!(" BIG {line} {h_x},{h_y} {t_x},{t_y} ");
//                 t_x = h_x - direction.0;
//                 t_y = h_y - direction.1;
//             } else if x_diff > 1 {
//                 t_x += direction.0;
//             } else if y_diff > 1 {
//                 t_y += direction.1;
//             }
//             tal_positions.insert((t_x, t_y));
//             // print_grid(&tal_positions);
//             // println!("XXXXXXXXXXXXXXXXXXXXXXXXXX");
//             //println!(" {line} {h_x},{h_y} {t_x},{t_y} ")
//         }
//     }
//     // print_grid(tal_positions.borrow());
//     println!("{}", &tal_positions.len());
//     // println!("{:?}", tal_positions);
// }

fn print_current_pos(positions: &[Knot], head: Knot) {
    let min_grid = -10;
    let max_grid = 20;
    for y in min_grid..max_grid {
        for x in min_grid..max_grid {
            if head.x == x && head.y == y {
                print!("*");
                continue;
            }
            if positions.iter().any(|x1| { x1.x == x && x1.y == y })
            {
                print!("#");
                continue;
            }
            print!(".")
        }
        println!();
    }
    for _ in min_grid..max_grid {
        print!("x");
    }
    println!();
}

fn print_grid(positions: &HashSet<(i32, i32)>) {
    let min_grid = 0;
    let max_grid = 20;
    for x in -min_grid..max_grid {
        for y in -min_grid..max_grid {
            if positions.contains(&(y, x))
            {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

//    for