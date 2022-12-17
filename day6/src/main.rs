use std::path::PathBuf;

fn main() {
    // let mut input_path = PathBuf::new();
    // input_path.push(std::env::current_dir().unwrap());
    // input_path.push("src/input.txt");

    // // Take in the input.txt file and create a vector where each element is a string of the line
    // let input_string = match std::fs::read_to_string(input_path) {
    //     Ok(s) => s,
    //     Err(e) => {
    //         println!("Error reading file: {}", e);
    //         std::process::exit(1);
    //     }
    // };

    // Read input file in as a &str
    let input_string = include_str!("input.txt");

    // Print the string
    println!("{}", input_string);

    // Iterate through each character in the string
    let mut start_of_packet_idx = 0;
    let mark_size = 4;
    for i in mark_size..input_string.len() {
        let mut unique = 1;
        // Print the index and the last four characters
        print!("{}: ", i);
        for j in 1..mark_size+1 {
            print!("{}", input_string.chars().nth(i-j).unwrap());
        }
        print!("\r\n");
        // Check if the last four characters are unique
        for j in 1..mark_size+1 {
            for k in 1..mark_size+1 {
                if j != k {
                    if input_string.chars().nth(i-j).unwrap() == input_string.chars().nth(i-k).unwrap() {
                        unique = 0;
                    }
                }
            }
        }
        if unique == 1 {
            start_of_packet_idx = i;
            break;
        }
    }

    // Print the index
    println!("{}", start_of_packet_idx);

    // Part 2
    let mut start_of_packet_idx = 0;
    let mark_size = 14;
    for i in mark_size..input_string.len() {
        let mut unique = 1;
        // Print the index and the last mark_size characters
        print!("{}: ", i);
        for j in 1..mark_size+1 {
            print!("{}", input_string.chars().nth(i-j).unwrap());
        }
        print!("\r\n");
        // Check if the last four characters are unique
        for j in 1..mark_size+1 {
            for k in 1..mark_size+1 {
                if j != k {
                    if input_string.chars().nth(i-j).unwrap() == input_string.chars().nth(i-k).unwrap() {
                        unique = 0;
                    }
                }
            }
        }
        if unique == 1 {
            start_of_packet_idx = i;
            break;
        }
    }

    // Print the index
    println!("{}", start_of_packet_idx);

}
