fn main() {
    // Get the current working directory
    let cwd = std::env::current_dir().unwrap();

    // Read in the input from the file
    let raw_input = std::fs::read_to_string(cwd.join("src/input.txt")).unwrap();

    // Convert the input into a vector of strings
    let input: Vec<&str> = raw_input.lines().collect();
    let x_len = input.len();
    let y_len = input[0].len();

    // // Print the input
    // for line in input.iter() {
    //     println!("{}", line);
    // }
    // print!("-----------------------------------------------\n");

    // Create a 2D array
    let mut grid = vec![vec![0; y_len]; x_len];

    // Fill the grid with the input
    for i in 0..x_len {
        for j in 0..y_len {
            grid[i][j] = input[i].chars().nth(j).unwrap() as u8;
        }
    }

    // Print the grid, with grid positions around the outside
    for i in 0..x_len {
        for j in 0..y_len {
            print!("{}", grid[i][j] as char);
        }
        print!("\n");
    }
    print!("-----------------------------------------------\n");

    let mut vis_trees: Vec<[u8;3]> = Vec::new();
    // For each element, determine if any value north, south, east, or west is greater
    for x in 1..(x_len-1) {
        for y in 1..(y_len-1) {
            let mut vis_north = true;
            let mut vis_south = true;
            let mut vis_east = true;
            let mut vis_west = true;

            // North
            for i in 0..x {
                if grid[x][y] <= grid[i][y] {
                    vis_north = false;
                    break;
                }
            }
            // South
            for i in (x+1)..x_len {
                if grid[x][y] <= grid[i][y] {
                    vis_south = false;
                    break;
                }
            }
            // East
            for i in 0..y {
                if grid[x][y] <= grid[x][i] {
                    vis_east = false;
                    break;
                }
            }
            // West
            for i in (y+1)..y_len {
                if grid[x][y] <= grid[x][i] {
                    vis_west = false;
                    break;
                }
            }

            if vis_north || vis_south || vis_east || vis_west {
                vis_trees.push([x as u8, y as u8, grid[x][y]]);
            }
        }
    }

    // Print the visible trees
    for tree in vis_trees.iter() {
        print!("({}, {}, {})\n", tree[0], tree[1], tree[2] as char);
    }

    // Print the number of visible trees
    let mut vis_trees_len = vis_trees.len() + x_len*2 + y_len*2 - 4;
    print!("Number of visible trees: {}\n", vis_trees_len);

    // Part 2
    struct Scene {
        tree_height: u8,
        x: u8,
        y: u8,
        view_dis_north: u32,
        view_dis_south: u32,
        view_dis_east: u32,
        view_dis_west: u32,
        scene_score: u32,
    }
    let mut scene_scores: Vec<Scene> = Vec::new();
    // For each element, determine if any value north, south, east, or west is greater
    for x in 1..(x_len-1) {
    // for x in 1..10 {
        for y in 1..(y_len-1) {
        // for y in 1..10 {
            let mut view_dis_north = 0;
            let mut view_dis_south = 0;
            let mut view_dis_east = 0;
            let mut view_dis_west = 0;

            println!("Height: {}, x: {}, y: {}", grid[x][y] as char, x, y);
            // North
            print!("North:");
            for i in (0..x).rev() {
                print!(" {}", grid[i][y] as char);
                if grid[x][y] <= grid[i][y] {
                    view_dis_north += 1;
                    break;
                }
                view_dis_north += 1;
            }
            print!(" -> {}\n", view_dis_north);
            // South
            print!("South:");
            for i in (x+1)..x_len {
                print!(" {}", grid[i][y] as char);
                if grid[x][y] <= grid[i][y] {
                    view_dis_south += 1;
                    break;
                }
                view_dis_south += 1;
            }
            print!(" -> {}\n", view_dis_south);
            // East
            print!("East:");
            for i in (0..y).rev() {
                print!(" {}", grid[x][i] as char);
                if grid[x][y] <= grid[x][i] {
                    view_dis_east += 1;
                    break;
                }
                view_dis_east += 1;
            }
            print!(" -> {}\n", view_dis_east);
            // West
            print!("West:");
            for i in (y+1)..y_len {
                print!(" {}", grid[x][i] as char);
                if grid[x][y] <= grid[x][i] {
                    view_dis_west += 1;
                    break;
                }
                view_dis_west += 1;
            }
            print!(" -> {}\n", view_dis_west);

            let scene_score: u32 = view_dis_north * view_dis_south * view_dis_east * view_dis_west;
            scene_scores.push(Scene {
                tree_height: grid[x][y],
                x: x as u8,
                y: y as u8,
                view_dis_north: view_dis_north,
                view_dis_south: view_dis_south,
                view_dis_east: view_dis_east,
                view_dis_west: view_dis_west,
                scene_score: scene_score,
            });
        }
    }

    // Print the scene scores
    // for scene in scene_scores.iter() {
    //     print!("({}, {}, {}, {}, {}, {}, {}, {})\n", scene.tree_height as char, scene.x, scene.y, scene.view_dis_north, scene.view_dis_south, scene.view_dis_east, scene.view_dis_west, scene.scene_score);
    // }

    // Find the max scene score
    let mut max_scene_score_idx = 0;
    for i in 0..scene_scores.len() {
        if scene_scores[i].scene_score > scene_scores[max_scene_score_idx].scene_score {
            max_scene_score_idx = i;
        }
    }

    // Print the max scene score
    print!("Max scene score: ({}, {}, {}, {}, {}, {}, {}, {})\n", scene_scores[max_scene_score_idx].tree_height as char, scene_scores[max_scene_score_idx].x, scene_scores[max_scene_score_idx].y, scene_scores[max_scene_score_idx].view_dis_north, scene_scores[max_scene_score_idx].view_dis_south, scene_scores[max_scene_score_idx].view_dis_east, scene_scores[max_scene_score_idx].view_dis_west, scene_scores[max_scene_score_idx].scene_score);
}
