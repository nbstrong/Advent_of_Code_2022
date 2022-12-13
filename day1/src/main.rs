fn main() {
    // Get list of numbers from file, including
    // a blank element for empty lines
    let numbers = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>();

    // Print the numbers
    println!("{:?}", numbers);

    // Go through the vector and add each number until a zero is reached. Created a new
    // vector with each element being the sums of the previous elements.
    let mut sums = Vec::new();
    let mut sum = 0;
    for number in numbers {
        sum += number;
        if number == 0 {
            sums.push(sum);
            sum = 0;
        }
    }

    // Print the sums
    println!("{:?}", sums);

    // Find the largest sum
    let largest = sums.iter().max().unwrap();

    // Print the largest sum
    println!("{}", largest);

    // Find the sum of the top 3 largest sums
    let mut sums = sums;
    sums.sort();
    sums.reverse();
    let top_three = sums[0] + sums[1] + sums[2];

    // Print the sum of the top 3 largest sums
    println!("{}", top_three);
}
