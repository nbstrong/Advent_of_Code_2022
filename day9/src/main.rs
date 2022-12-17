const PRINT: bool = false;

fn main() {
    // Get the current working directory
    let cwd = std::env::current_dir().unwrap();

    // Read in the input from the file
    let raw_input = std::fs::read_to_string(cwd.join("src/input.txt")).unwrap();

    // Convert the input into a vector of strings
    let input: Vec<&str> = raw_input.lines().collect();

    const ROPE_LEN: usize = 10;

    struct knot {
        x: i32,
        y: i32,
    }
    struct System {
        rope: [knot; ROPE_LEN]
    }
    impl System {
        fn new() -> System {
            System {
                rope: [
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                    knot { x: 0, y: 0 },
                ]
            }
        }
    }

    let mut system = System::new();

    let mut visited: Vec<(i32, i32)> = Vec::new();
    visited.push((system.rope[0].x, system.rope[0].y));

    const PLEN: i32 = 5;
    if PRINT {
        println!("== Initial Position ==");
        // Print the grid and rope
        for y in (system.rope[ROPE_LEN-1].y-PLEN)..=(system.rope[ROPE_LEN-1].y+PLEN) {
            for x in (system.rope[ROPE_LEN-1].x-PLEN)..=(system.rope[ROPE_LEN-1].x+PLEN) {
                if system.rope.iter().any(|p| p.x == x && p.y == y) {
                    // Print the index
                    print!("{}", system.rope.iter().position(|p| p.x == x && p.y == y).unwrap());
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }


    let mut counter = 0;
    for line in input {
        // Split the line via whitespace
        let line_vec: Vec<&str> = line.split_whitespace().collect();

        // Get the direction and distance
        let direction = line_vec[0].chars().nth(0).unwrap();
        let distance = line_vec[1].parse::<i32>().unwrap();

        // Print the grid
        println!("== {}, Len: {} ==", line, visited.len());
        println!();

        for _ in 0..distance {
            match direction {
                'U' => system.rope[ROPE_LEN-1].y -= 1,
                'D' => system.rope[ROPE_LEN-1].y += 1,
                'L' => system.rope[ROPE_LEN-1].x -= 1,
                'R' => system.rope[ROPE_LEN-1].x += 1,
                _ => panic!("Invalid direction"),
            }

            for i in (0..=ROPE_LEN-2).rev() {
                if (system.rope[i+1].x - system.rope[i].x).abs() > 1 && (system.rope[i+1].y - system.rope[i].y).abs() == 0
                || (system.rope[i+1].x - system.rope[i].x).abs() == 0 && (system.rope[i+1].y - system.rope[i].y).abs() > 1
                {
                    // Move the tail towards the head
                    if system.rope[i+1].x > system.rope[i].x {
                        system.rope[i].x += 1;
                    } else if system.rope[i+1].x < system.rope[i].x {
                        system.rope[i].x -= 1;
                    } else if system.rope[i+1].y > system.rope[i].y {
                        system.rope[i].y += 1;
                    } else if system.rope[i+1].y < system.rope[i].y {
                        system.rope[i].y -= 1;
                    }
                }
                else if (system.rope[i+1].x - system.rope[i].x).abs() > 1 || (system.rope[i+1].y - system.rope[i].y).abs() > 1 {
                    // Move the tail towards the head
                    if system.rope[i+1].x > system.rope[i].x && system.rope[i+1].y > system.rope[i].y {
                        system.rope[i].x += 1;
                        system.rope[i].y += 1;
                    } else if system.rope[i+1].x < system.rope[i].x && system.rope[i+1].y < system.rope[i].y {
                        system.rope[i].x -= 1;
                        system.rope[i].y -= 1;
                    } else if system.rope[i+1].x > system.rope[i].x && system.rope[i+1].y < system.rope[i].y {
                        system.rope[i].x += 1;
                        system.rope[i].y -= 1;
                    } else if system.rope[i+1].x < system.rope[i].x && system.rope[i+1].y > system.rope[i].y {
                        system.rope[i].x -= 1;
                        system.rope[i].y += 1;
                    }
                }
            }

            // Check if the tail is not in the visited list
            if !visited.contains(&(system.rope[0].x, system.rope[0].y)) {
                visited.push((system.rope[0].x, system.rope[0].y));
            }

            // // Check if the head is more than 1 away from the tail
            // if (system.head.x - system.tail.x).abs() > 1 && (system.head.y - system.tail.y).abs() == 0
            //     || (system.head.x - system.tail.x).abs() == 0 && (system.head.y - system.tail.y).abs() > 1
            // {
            //     // Move the tail towards the head
            //     if system.head.x > system.tail.x {
            //         system.tail.x += 1;
            //     } else if system.head.x < system.tail.x {
            //         system.tail.x -= 1;
            //     } else if system.head.y > system.tail.y {
            //         system.tail.y += 1;
            //     } else if system.head.y < system.tail.y {
            //         system.tail.y -= 1;
            //     }
            // }
            // else if (system.head.x - system.tail.x).abs() > 1 || (system.head.y - system.tail.y).abs() > 1 {
            //     // Move the tail towards the head
            //     if system.head.x > system.tail.x && system.head.y > system.tail.y {
            //         system.tail.x += 1;
            //         system.tail.y += 1;
            //     } else if system.head.x < system.tail.x && system.head.y < system.tail.y {
            //         system.tail.x -= 1;
            //         system.tail.y -= 1;
            //     } else if system.head.x > system.tail.x && system.head.y < system.tail.y {
            //         system.tail.x += 1;
            //         system.tail.y -= 1;
            //     } else if system.head.x < system.tail.x && system.head.y > system.tail.y {
            //         system.tail.x -= 1;
            //         system.tail.y += 1;
            //     }
            // }

            // // Check if the tail is not in the visited list
            // if !visited.contains(&(system.tail.x, system.tail.y)) {
            //     visited.push((system.tail.x, system.tail.y));
            // }

            if PRINT {
                // Print the grid and rope
                for y in (system.rope[ROPE_LEN-1].y-PLEN)..=(system.rope[ROPE_LEN-1].y+PLEN) {
                    for x in (system.rope[ROPE_LEN-1].x-PLEN)..=(system.rope[ROPE_LEN-1].x+PLEN) {
                        if system.rope.iter().any(|p| p.x == x && p.y == y) {
                            // Print the index
                            print!("{}", system.rope.iter().position(|p| p.x == x && p.y == y).unwrap());
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!();
            }
        }

        // counter += 1;
        // if counter > 20 {
        //     break;
        // }
    }

    // Print the length of the visited list
    println!("{}", visited.len());
}
