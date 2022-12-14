use std::path::PathBuf;

fn main() {
    let mut input_path = PathBuf::new();
    input_path.push(std::env::current_dir().unwrap());
    input_path.push("src/input.txt");

    // Take in the input.txt file and create a vector where each element is a string of the line
    let input_string = match std::fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Create a vector of lines from the input file
    let input_lines: Vec<&str> = input_string.lines().collect();

    // Separate the lines into map and procedure
    let map_vec = &input_lines[0..9];
    let procedure_vec = &input_lines[10..input_lines.len()];

    // Print the map
    for line in map_vec {
        println!("{}", line);
    }

    // Push the empty stacks into a vector
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 1..10 {
        stacks.push(Vec::new());
    }

    // Create a vector of the stacks
    for line in &map_vec[0..map_vec.len()-1] {
        // Push the first character to the stack
        // then the 5th character, then the 9th character, etc
        // unless it is a space then don't.
        for i in 0..line.len() {
            if ((i as i8)-1) % 4 == 0 {
                if line.chars().nth(i).unwrap() != ' ' {
                    stacks[((((i as i8)-1)/4)) as usize].push(line.chars().nth(i).unwrap());
                }
            }
        }
    }

    // Reverse each element in the stacks
    for stack in &mut stacks {
        stack.reverse();
    }

    let orig_stacks = stacks.clone();

    // Print the index and stack
    for (i, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", i+1, stack);
    }
    // Print a separator
    println!("---------------------");

    for command in procedure_vec {
        let word_vec = command.split_whitespace().collect::<Vec<&str>>();
        let move_amt = word_vec[1].parse::<u8>().unwrap();
        let from_stack = word_vec[3].parse::<usize>().unwrap()-1;
        let to_stack = word_vec[5].parse::<usize>().unwrap()-1;

        // Move the top elements from one stack to another n times
        for _ in 0..move_amt {
            let top_element = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(top_element);
        }
    }

    // Print the index and stack
    for (i, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", i+1, stack);
    }
    // Print a separator
    println!("---------------------");

    // Part 2

    // Reset the stack vector to the original stacks
    stacks = orig_stacks.clone();

    for command in procedure_vec {
        let word_vec = command.split_whitespace().collect::<Vec<&str>>();
        let move_amt = word_vec[1].parse::<usize>().unwrap();
        let from_stack = word_vec[3].parse::<usize>().unwrap()-1;
        let to_stack = word_vec[5].parse::<usize>().unwrap()-1;
        let move_idx = stacks[from_stack].len()-move_amt;

        // Move the top n elements from one stack to another
        let mut temp_stack = stacks[from_stack].split_off(move_idx);
        stacks[to_stack].append(&mut temp_stack);
    }

    // Print the index and stack
    for (i, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", i+1, stack);
    }
}
