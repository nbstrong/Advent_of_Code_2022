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

    // Split the string by newline and collect it into a vector
    let input = input_string.split("\n")
        .collect::<Vec<&str>>();

    // Drop the last element of the vector
    let input = &input[..input.len() - 1];

    // Print the length
    println!("Length: {}", input.len());
    // Print the first and last 3 elements
    println!("First 3 elements: {:?}", &input[0..3]);
    println!("Last 3 elements: {:?}", &input[input.len() - 3..input.len()]);

    // Take each element of the vector and split it by comma and push it to a new vector
    let mut new_input = Vec::new();
    for x in input {
        let split = x.split(",").collect::<Vec<&str>>();
        new_input.push(split);
    }

    // Print the length
    println!("Length: {}", new_input.len());
    // Print the first and last 3 elements
    println!("First 3 elements: {:?}", &new_input[0..3]);
    println!("Last 3 elements: {:?}", &new_input[new_input.len() - 3..new_input.len()]);

    // Convert each element into a range
    let mut range_pairs = Vec::new();
    for x in new_input {
        let mut pair = Vec::new();
        for y in x {
        let range = y.split("-").collect::<Vec<&str>>();
        // Convert range into a Range
        let range = range[0].parse::<u32>().unwrap()..=range[1].parse::<u32>().unwrap();
        pair.push(range);
        }
        range_pairs.push(pair);
    }

    // Print the length
    println!("Length: {}", range_pairs.len());
    // Print the first and last 3 elements
    println!("First 3 elements: {:?}", &range_pairs[0..3]);
    println!("Last 3 elements: {:?}", &range_pairs[range_pairs.len() - 3..range_pairs.len()]);

    // For each element in the vector determine if one range is fully inside another
    let mut inside = Vec::new();
    for pair in &range_pairs {
        if pair[0].contains(&pair[1].start()) && pair[0].contains(&pair[1].end()) {
            inside.push(pair);
        }
        else if pair[1].contains(&pair[0].start()) && pair[1].contains(&pair[0].end()) {
            inside.push(pair);
        }
    }

    // Print the length
    println!("Length: {}", inside.len());
    // Print the first and last 3 elements
    println!("First 3 elements: {:?}", &inside[0..3]);
    println!("Last 3 elements: {:?}", &inside[inside.len() - 3..inside.len()]);

    // Part 2
    // Determine how many ranges overlap at all
    let mut overlap = Vec::new();
    for pair in &range_pairs {
        if pair[0].contains(&pair[1].start()) || pair[0].contains(&pair[1].end()) {
            overlap.push(pair);
        }
        else if pair[1].contains(&pair[0].start()) || pair[1].contains(&pair[0].end()) {
            overlap.push(pair);
        }
    }

    // Print the length
    println!("Length: {}", overlap.len());
    // Print the first and last 3 elements
    println!("First 3 elements: {:?}", &overlap[0..3]);
    println!("Last 3 elements: {:?}", &overlap[overlap.len() - 3..overlap.len()]);

}
