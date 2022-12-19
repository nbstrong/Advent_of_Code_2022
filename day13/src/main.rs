
// static raw_input: &str = include_str!("input.txt");
static raw_input: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum NumOrArray {
    Num(i32),
    Array(Box<Vec<NumOrArray>>),
}
impl NumOrArray {
    fn len(&self) -> usize {
        match self {
            NumOrArray::Num(_) => 1,
            NumOrArray::Array(v) => v.len(),
        }
    }
}

enum ThreeState {
    True,
    False,
    Unknown,
}
impl ThreeState {
    pub fn resolve(&self) -> bool {
        match self {
            ThreeState::True => true,
            ThreeState::False => false,
            ThreeState::Unknown => true,
        }
    }
}

fn main() {
    // Separate the input into lines
    let lines: Vec<&str> = raw_input.lines().collect();

    let mut signal: Vec<Box<Vec<NumOrArray>>> = Vec::new();
    let mut left = String::new();
    let mut right = String::new();
    // Create an array of booleans for whether they are in the correct order.
    let mut correct_order: Vec<ThreeState> = Vec::new();
    // Enumerate the lines
    for (i, line) in lines.iter().enumerate() {
        if i % 3 == 0 {
            // Convert the &str to a String
            left = line.to_string();
        }
        else if i % 3 == 1 {
            right = line.to_string();
        }
        else {
            // Convert the string of [1,[2,3],4] to int vectors
            let left_array = string_to_vec(&left);
            let right_array = string_to_vec(&right);

            // Rules for Correct Order
            // 1. Integers in the left side must always be <= integers in the right side
            // 2. Array lengths in the left side must always be <= array lengths in the right side
            correct_order.push(compare_arrays(&left_array, &right_array));

            // Push the left and right arrays to the signal
            signal.push(left_array);
            signal.push(right_array);
        }
    }

    // Print the number of unknowns
    let mut unknown_count = 0;
    for i in 0..correct_order.len() {
        match correct_order[i] {
            ThreeState::Unknown => {
                unknown_count += 1;
            }
            _ => {}
        }
    }
    println!("Unknowns: {}", unknown_count);

    // Resolve ThreeState logic to bools
    let mut correct_ord_bool: Vec<bool> = Vec::new();
    for i in 0..correct_order.len() {
        match correct_order[i] {
            ThreeState::True => {
                correct_ord_bool.push(true);
            }
            ThreeState::False => {
                correct_ord_bool.push(false);
            }
            ThreeState::Unknown => {
                correct_ord_bool.push(true);
            }
        }
    }

    // Print the result
    println!("{:?}", correct_ord_bool);

    // Sum up the indices of the correct order
    let mut sum = 0;
    for i in 0..correct_ord_bool.len() {
        if correct_ord_bool[i] {
            sum += i+1;
        }
    }

    // Print the sum
    println!("{}", sum);

    // Part 2
    // Sort the signal vector via bubble sort
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..signal.len()-1 {
            if compare_arrays(&signal[i], &signal[i+1]).resolve() {
                signal.swap(i, i+1);
                swapped = true;
            }
        }
    }

    signal.reverse();

    // Find the indices of [[2]] and [[6]]
    let mut two_index = 0;
    let mut six_index = 0;
    for i in 0..signal.len() {
        if signal[i].len() == 1 {
            match &signal[i][0] {
                NumOrArray::Num(n) => {
                    if *n == 2 {
                        two_index = i;
                    }
                    else if *n == 6 {
                        six_index = i;
                    }
                }
                _ => {}
            }
        }
    }

    // // Print each vec in the signal in the form [1,[2,3],4]
    for i in 0..signal.len() {
        print!("{}:", i+1);
        print_vec_num_or_array(&signal[i]);
    }

    // Print the indices
    println!("2: {}", two_index);
    println!("6: {}", six_index);

    // Print the multiplicand of the two indices
    println!("Multiplicand: {}", two_index * six_index);


}

// Parse the string until you reach a comma or a bracket
// If you reach a comma, push the number to the vector
// If you reach a bracket, recurse
// Once returned, continue after the closing bracket
fn string_to_vec(s: &String) -> Box<Vec<NumOrArray>> {
    let mut vec: Box<Vec<NumOrArray>> = Box::new(Vec::new());
    let mut string = String::new();
    let mut num_or_array: Box<NumOrArray> = Box::new(NumOrArray::Num(0));
    let mut i = 0;
    // Strip begin and end brackets
    let s = &s[1..s.len() - 1];
    loop {
        let c = match s.chars().nth(i) {
            Some(c) => c,
            None => ' ',
        };
        if c == ',' || i >= s.len() {
            if string.len() > 0 {
                vec.push(NumOrArray::Num(string.parse().unwrap()));
                string = String::new();
            }
            if i >= s.len() {
                break;
            }
        }
        else if c == '[' {
            let mut j = i + 1;
            let mut bracket_count = 1;
            while bracket_count > 0 {
                let c = s.chars().nth(j).unwrap();
                if c == '[' {
                    bracket_count += 1;
                }
                else if c == ']' {
                    bracket_count -= 1;
                }
                j += 1;
            }
            vec.push(NumOrArray::Array(string_to_vec(&s[i..j].to_string())));
            i = j-1;
        }
        else {
            string.push(c);
        }
        i += 1;
    }
    vec
}

fn compare_arrays(left: &Vec<NumOrArray>, right: &Vec<NumOrArray>) -> ThreeState {
    for i in 0..std::cmp::max(left.len(), right.len()) {
        if(i >= left.len() && i < right.len()) {
            // println!("{:?} < {:?} so true", left, right);
            return ThreeState::True;
        }
        else if(i < left.len() && i >= right.len()) {
            // println!("{:?} > {:?} so false", left, right);
            return ThreeState::False;
        }
        else {
            match (&left[i], &right[i]) {
                (NumOrArray::Num(l), NumOrArray::Num(r)) => {
                    // println!("{}:{}", l, r);
                    if l > r {
                        // println!("{} > {} so false", l, r);
                        return ThreeState::False;
                    }
                    else if l < r {
                        // println!("{} < {} so true", l, r);
                        return ThreeState::True;
                    }
                }
                (NumOrArray::Array(l), NumOrArray::Array(r)) => {
                    // println!("{:?} == {:?}", l, r);
                    match compare_arrays(l, r) {
                        ThreeState::True => {
                            return ThreeState::True;
                        }
                        ThreeState::False => {
                            return ThreeState::False;
                        }
                        ThreeState::Unknown => {}
                    }
                }
                (NumOrArray::Num(l), NumOrArray::Array(r)) => {
                    // println!("{:?} == {:?}", l, r);
                    match compare_arrays(&vec![NumOrArray::Num(*l)], r) {
                        ThreeState::True => {
                            return ThreeState::True;
                        }
                        ThreeState::False => {
                            return ThreeState::False;
                        }
                        ThreeState::Unknown => {}
                    }
                }
                (NumOrArray::Array(l), NumOrArray::Num(r)) => {
                    // println!("{:?} == {:?}", l, r);
                    match compare_arrays(l, &vec![NumOrArray::Num(*r)]) {
                        ThreeState::True => {
                            return ThreeState::True;
                        }
                        ThreeState::False => {
                            return ThreeState::False;
                        }
                        ThreeState::Unknown => {}
                    }
                }
                _ => {}
            }
        }
    }
    return ThreeState::Unknown;
}

fn print_num_or_array(n: &NumOrArray) {
    match n {
        NumOrArray::Num(n) => {
            print!("{}", n);
        }
        NumOrArray::Array(a) => {
            print!("[");
            for i in 0..a.len() {
                print_num_or_array(&a[i]);
                if i < a.len() - 1 {
                    print!(",");
                }
            }
            print!("]");
        }
    }
}

fn print_vec_num_or_array(v: &Vec<NumOrArray>) {
    print!("[");
    for i in 0..v.len() {
        print_num_or_array(&v[i]);
        if i < v.len() - 1 {
            print!(",");
        }
    }
    print!("]");
    println!();
}