extern crate pathfinding;

use pathfinding::prelude::dijkstra;

// static raw_input: &str = include_str!("input_small.txt");
static raw_input: &str = include_str!("input.txt");

fn main() {

    let lines: Vec<&str> = raw_input.lines().collect::<Vec<&str>>();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let row_len: usize = grid[0].len();
    let col_len: usize = grid.len();

    // Transpose the grid
    let grid: Vec<Vec<char>> = (0..row_len).map(|i| grid.iter().map(|row| row[i]).collect()).collect();

    println!("Row length: {}", row_len);
    println!("Column length: {}", col_len);

    // Print the grid
    for y in 0..col_len {
        for x in 0..row_len {
            print!("{}", grid[x][y]);
        }
        println!("");
    }

    #[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(i32, i32);

    // Determine the xy position of S and E
    let mut start: Pos = Pos(0, 0);
    let mut end: Pos = Pos(0, 0);
    for y in 0..col_len {
        for x in 0..row_len {
            if grid[x][y] == 'S' {
                start = Pos(x as i32, y as i32);
            } else if grid[x][y] == 'E' {
                end = Pos(x as i32, y as i32);
            }
        }
    }

    println!("Start: {:?}", start);
    println!("End: {:?}", end);

    fn cost(a: &Pos, b: &Pos, grid: &Vec<Vec<char>>) -> usize {
        let row_len = grid.len();
        let col_len = grid[0].len();
        // Print the row and col length
        // println!("Row length: {}", row_len);
        // println!("Column length: {}", col_len);

        // Check if the positions are in bounds
        if b.0 >= 0 && b.0 <= ((row_len-1) as i32) && b.1 >= 0 && b.1 <= ((col_len-1) as i32) {
            // println!("x: {}, y: {}", a.0, a.1);
            // println!("x: {}, y: {}", b.0, b.1);
            let mut a_char = grid[a.0 as usize][a.1 as usize];
            let mut b_char = grid[b.0 as usize][b.1 as usize];
            match a_char {
                'S' => a_char = 'a',
                'E' => a_char = 'z',
                _ => (),
            }
            match b_char {
                'S' => b_char = 'a',
                'E' => b_char = 'z',
                _ => (),
            }
            let a_int = a_char as usize - 97;
            let b_int = b_char as usize - 97;

            if (b_int as i32) - (a_int as i32) > 1 {
                // println!("x: {}, y: {}, {} => {}, cost 9999", b.0, b.1, a_char, b_char);
                return 9999 as usize;
            }
            else {
                // println!("x: {}, y: {}, {} => {}, cost 1", b.0, b.1, a_char, b_char);
                return 1 as usize
            }

        }
        return 9999 as usize;
    }

    // If you have a 5x5 grid
    // abcde
    // fghij
    // klmno
    // pqrst
    // uvwxy
    // The neighbors of m at position (2, 2) are
    // h (2, 1), r (2, 3), l (1, 2), n (3, 2), g (1, 1), i (3, 1), q (1, 3), s (3, 3)
    impl Pos {
        fn successors(&self, grid: &Vec<Vec<char>>) -> Vec<(Pos, usize)> {
            let &Pos(x, y) = self;
            // println!("====================");
            // println!("x: {}, y: {}: {}", x, y, grid[x as usize][y as usize]);
            // Print the 3x3 grid around the Pos
            // for y in (y-1)..(y+2) {
            //     for x in (x-1)..(x+2) {
            //         if x < 0 || x >= (grid.len() as i32) || y < 0 || y >= (grid[0].len() as i32) {
            //             print!(" ");
            //         }
            //         else {
            //             print!("{}", grid[x as usize][y as usize]);
            //         }
            //     }
            //     println!("");
            // }
            let mut neighbors: Vec<(Pos, usize)> = vec![Pos(x, y-1), Pos(x, y+1), Pos(x-1, y), Pos(x+1, y)]
            // Pos(x-1, y-1), Pos(x+1, y-1), Pos(x-1, y+1), Pos(x+1, y+1)]
                .into_iter()
                .map(|p| (p, cost(self, &p, &grid)))
                .collect();

            // Remove neighbors that are out of bounds
            neighbors.retain(|&(p, _)| p.0 >= 0 && p.0 < (grid.len() as i32) && p.1 >= 0 && p.1 < (grid[0].len() as i32));
            // Remove neighbors that have a cost of 9999
            neighbors.retain(|&(_, c)| c != 9999);

            neighbors
        }
    }

    // Find the shortest path from S to E
    let path = dijkstra(&start, |p| p.successors(&grid), |p| *p == end);

    // Print the path
    println!("{:?}", path);
    println!();

    // Print the original map
    for y in 0..col_len {
        for x in 0..row_len {
            print!("{}", grid[x][y]);
        }
        println!("");
    }
    println!();

    // Print the map with the path
    for y in 0..col_len {
        for x in 0..row_len {
            if path.as_ref().unwrap().0.contains(&Pos(x as i32, y as i32)) {
                // Save the index of the contained position
                let index = path.as_ref().unwrap().0.iter().position(|&r| r == Pos(x as i32, y as i32)).unwrap();

                // If the next position is below the current position, print a down arrow
                if index < path.as_ref().unwrap().0.len() - 1 {
                    if path.as_ref().unwrap().0[index + 1].1 > y as i32 {
                        print!("v");
                    }
                    // If the next position is above the current position, print an up arrow
                    else if path.as_ref().unwrap().0[index + 1].1 < y as i32 {
                        print!("^");
                    }
                    // If the next position is to the right of the current position, print a right arrow
                    else if path.as_ref().unwrap().0[index + 1].0 > x as i32 {
                        print!(">");
                    }
                    // If the next position is to the left of the current position, print a left arrow
                    else if path.as_ref().unwrap().0[index + 1].0 < x as i32 {
                        print!("<");
                    }
                }

            } else {
                print!("{}", grid[x][y]);
            }
        }
        println!("");
    }

    // Print the path length
    println!("Path length: {}", path.unwrap().0.len()-1);

    // Part 2
    // Find all the positions that are of elevation a
    let mut a_positions: Vec<Pos> = Vec::new();
    for y in 0..col_len {
        for x in 0..row_len {
            if grid[x][y] == 'a' {
                a_positions.push(Pos(x as i32, y as i32));
            }
        }
    }

    // Print the length of the vector
    println!("a_positions length: {}", a_positions.len());

    //Find the shorted path from each position and append the path length-1 to a vector
    let mut paths: Vec<Vec<Pos>> = Vec::new();
    for a in a_positions {
        let path = dijkstra(&a, |p| p.successors(&grid), |p| *p == end);
        match path {
            Some(p) => paths.push(p.0),
            None => (),
        }
    }


    // Print the length of paths
    println!("paths length: {}", paths.len());

    // Print all of the path lengths
    // for path in &paths {
    //     println!("Path length: {}", path.len()-1);
    // }

    // Find the min length path
    let min_path = paths.iter().min_by_key(|p| p.len());

    // Print the original map
    for y in 0..col_len {
        for x in 0..row_len {
            print!("{}", grid[x][y]);
        }
        println!("");
    }
    println!();

    // Print the map with the min path
    for y in 0..col_len {
        for x in 0..row_len {
            if min_path.as_ref().unwrap().contains(&Pos(x as i32, y as i32)) {
                // Save the index of the contained position
                let index = min_path.as_ref().unwrap().iter().position(|&r| r == Pos(x as i32, y as i32)).unwrap();

                // If the next position is below the current position, print a down arrow
                if index < min_path.as_ref().unwrap().len() - 1 {
                    if min_path.as_ref().unwrap()[index + 1].1 > y as i32 {
                        print!("v");
                    }
                    // If the next position is above the current position, print an up arrow
                    else if min_path.as_ref().unwrap()[index + 1].1 < y as i32 {
                        print!("^");
                    }
                    // If the next position is to the right of the current position, print a right arrow
                    else if min_path.as_ref().unwrap()[index + 1].0 > x as i32 {
                        print!(">");
                    }
                    // If the next position is to the left of the current position, print a left arrow
                    else if min_path.as_ref().unwrap()[index + 1].0 < x as i32 {
                        print!("<");
                    }
                }

            } else {
                print!("{}", grid[x][y]);
            }
        }
        println!("");
    }


    // Print the shortest path length
    println!("Shortest path length: {}", min_path.unwrap().len()-1);
}
