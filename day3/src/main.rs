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

    let input = input_string.split("\n")
        .collect::<Vec<&str>>();

    // Drop the last element
    let input = input[0..input.len() - 1].to_vec();

    // Print the index and the element of the vector
    for (i, x) in input.iter().enumerate() {
        println!("{}: {}", i, x);
    }

    // Create a vector where each element are two strings. Each string is the first and second half of the
    // line split by length.
    let half_strings = input.iter().map(|x| {
        // Split the string in half
        let split = x.split_at(x.len() / 2);
        // Return the two strings
        (split.0, split.1)
    }).collect::<Vec<(&str, &str)>>();

    // Print the index and the element of the vector
    for (i, x) in half_strings.iter().enumerate() {
        println!("{}: {:?}", i, x);
    }

    // Create a vector where each element is a single char and that char is the char common between both strings
    let common_chars = half_strings.iter().map(|x| {
        // Create an iterator of chars from the first string
        let first_chars = x.0.chars();
        // Create an iterator of chars from the second string
        let second_chars = x.1.chars();

        // Find the char that exists in both strings
        let common_chars = first_chars.filter(|x| {
            second_chars.clone().any(|y| y == *x)
        }).collect::<Vec<char>>();

        // Print the index and the common chars
        println!("{}, {}: {:?}", x.0, x.1, common_chars);

        // Return the first char in the vector of common chars
        common_chars[0]
    }).collect::<Vec<char>>();

    // Convert the char vector into a score vector where
    // a..z = 1..26 and A..Z = 27..52
    let scores = common_chars.iter().map(|x| {
        match x {
            'a'..='z' => *x as u8 - 96,
            'A'..='Z' => *x as u8 - 38,
            _ => 0,
        }
    }).collect::<Vec<u8>>();

    // Sum the scores
    let mut sum: u32 = 0;
    for score in scores {
        sum += score as u32;
    }

    // Print the sum
    println!("{}", sum);

    // Part 2
    let input = input_string.split("\n")
        .collect::<Vec<&str>>();

    // Drop the last element
    let input = input[0..input.len() - 1].to_vec();

    // Collect every 3 elements into a vector and put them into a new vector
    let mut input = input.chunks(3)
        .map(|x| {
            let mut temp = Vec::new();
            temp.push(x[0]);
            temp.push(x[1]);
            temp.push(x[2]);
            temp
        }).collect::<Vec<Vec<&str>>>();

    // Print the first 3 elements
    for i in 0..3 {
        println!("{:?}", input[i]);
    }

    // Find the char common between all 3 strings and put that char into a vector
    let common_chars = input.iter().map(|x| {
        // Create an iterator of chars from the first string
        let first_chars = x[0].chars();
        // Create an iterator of chars from the second string
        let second_chars = x[1].chars();
        // Create an iterator of chars from the third string
        let third_chars = x[2].chars();

        // Find the char that exists in all strings
        let common_chars = first_chars.filter(|x| {
            second_chars.clone().any(|y| y == *x) && third_chars.clone().any(|z| z == *x)
        }).collect::<Vec<char>>();

        // Print the index and the common chars
        println!("{}, {}, {}: {:?}", x[0], x[1], x[2], common_chars);

        // Return the first char in the vector of common chars
        common_chars[0]
    }).collect::<Vec<char>>();

    // Convert the char vector into a score vector where
    // a..z = 1..26 and A..Z = 27..52
    let scores = common_chars.iter().map(|x| {
        match x {
            'a'..='z' => *x as u8 - 96,
            'A'..='Z' => *x as u8 - 38,
            _ => 0,
        }
    }).collect::<Vec<u8>>();

    // Sum the scores
    let mut sum: u32 = 0;
    for score in scores {
        sum += score as u32;
    }

    // Print the sum
    println!("{}", sum);

}
