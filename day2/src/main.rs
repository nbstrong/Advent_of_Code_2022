fn main() {
    // Read the file and create two vectors, where each element
    // is the first and second column of the file, respectively.
    let (raw_theirs, raw_mine) : (Vec<char>, Vec<char>) = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (split.next().unwrap().chars().next().unwrap(),
             split.next().unwrap().chars().next().unwrap())
        })
        .unzip();

    // Convert A to "rock", B to "paper", and C to "scissors"
    let theirs = raw_theirs.iter().map(|c| match c {
        'A' => "rock",
        'B' => "paper",
        'C' => "scissors",
        _ => panic!("Invalid character"),
    }).collect::<Vec<&str>>();

    // Convert X to "rock", Y to "paper", and Z to "scissors"
    let mine = raw_mine.iter().map(|c| match c {
        'X' => "rock",
        'Y' => "paper",
        'Z' => "scissors",
        _ => panic!("Invalid character"),
    }).collect::<Vec<&str>>();

    // Convert the mine and yours vectors to a vector of outcomes
    let outcome = mine.iter().zip(theirs.iter()).map(|(mine, theirs)| {
        match (mine, theirs) {
            (&"rock",     &"rock")     => "tie",
            (&"rock",     &"paper")    => "lose",
            (&"rock",     &"scissors") => "win",
            (&"paper",    &"rock")     => "win",
            (&"paper",    &"paper")    => "tie",
            (&"paper",    &"scissors") => "lose",
            (&"scissors", &"rock")     => "lose",
            (&"scissors", &"paper")    => "win",
            (&"scissors", &"scissors") => "tie",
            _ => panic!("Invalid combination"),
        }
    }).collect::<Vec<&str>>();

    // Convert the outcome vector to a outcome_score vector
    let outcome_score = outcome.iter().map(|c| match c {
        &"win" => 6,
        &"lose" => 0,
        &"tie" => 3,
        _ => panic!("Invalid outcome"),
    }).collect::<Vec<i32>>();

    // Convert the mine vector to a mine_score vector
    let mine_score = mine.iter().map(|c| match c {
        &"rock" => 1,
        &"paper" => 2,
        &"scissors" => 3,
        _ => panic!("Invalid mine"),
    }).collect::<Vec<i32>>();

    // Create a vector that is the sum of the outcome_score and mine_score vectors
    let total_score = outcome_score.iter().zip(mine_score.iter()).map(|(outcome_score, mine_score)| {
        outcome_score + mine_score
    }).collect::<Vec<i32>>();

    // Sum up the total_score vector
    let total_score = total_score.iter().fold(0, |sum, x| sum + x);

    // Print the total score
    println!("{}", total_score);

    // Part 2
    // Convert X to lose, Y to tie, and Z to win
    let outcome = raw_mine.iter().map(|c| match c {
        'X' => "lose",
        'Y' => "tie",
        'Z' => "win",
        _ => panic!("Invalid character"),
    }).collect::<Vec<&str>>();

    // For each element in the theirs and mine vector, determine what to throw to
    // get the correct outcome
    let mine = theirs.iter().zip(outcome.iter()).map(|(theirs, outcome)| {
        match (theirs, outcome) {
            (&"rock",     &"lose") => "scissors",
            (&"rock",     &"tie")  => "rock",
            (&"rock",     &"win")  => "paper",
            (&"paper",    &"lose") => "rock",
            (&"paper",    &"tie")  => "paper",
            (&"paper",    &"win")  => "scissors",
            (&"scissors", &"lose") => "paper",
            (&"scissors", &"tie")  => "scissors",
            (&"scissors", &"win")  => "rock",
            _ => panic!("Invalid combination"),
        }
    }).collect::<Vec<&str>>();

    // Convert the mine vector into a mine_score vector
    let mine_score = mine.iter().map(|c| match c {
        &"rock" => 1,
        &"paper" => 2,
        &"scissors" => 3,
        _ => panic!("Invalid mine"),
    }).collect::<Vec<i32>>();

    // Convert the outcome vector into an outcome_score vector
    let outcome_score = outcome.iter().map(|c| match c {
        &"win" => 6,
        &"lose" => 0,
        &"tie" => 3,
        _ => panic!("Invalid outcome"),
    }).collect::<Vec<i32>>();

    // Create a vector that is the sum of the outcome_score and mine_score vectors
    let total_score = outcome_score.iter().zip(mine_score.iter()).map(|(outcome_score, mine_score)| {
        outcome_score + mine_score
    }).collect::<Vec<i32>>();

    // Sum up the total_score vector
    let total_score = total_score.iter().fold(0, |sum, x| sum + x);

    // Print the total score
    println!("{}", total_score);


}
