fn main() {
    // Get the current working directory
    let cwd = std::env::current_dir().unwrap();

    // Read in the input from the file
    let raw_input = std::fs::read_to_string(cwd.join("src/input.txt")).unwrap();

    // Get an iterator of the lines
    let mut input = raw_input.lines();

    // Infinitely loop
    let mut counter = 1;
    let mut target = 1;
    let mut reg_x = 1;
    let mut command: &str = "noop";
    let mut op: i32 = 0;
    let mut signal_strength = 0;
    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut crt_array: [char; 40*6] = ['.'; 40*6];
    let mut crt_pos = 0;
    loop {
        if counter == target {
            if command == "addx" {
                reg_x += op;
            }
            // Get a line from the input
            let line = match input.next() {
                Some(line) => line,
                None => break,
            };

            // Parse the command
            let cmd_line = line.split(' ').collect::<Vec<_>>();
            command = cmd_line[0];

            if command != "noop" {
                op = cmd_line[1].parse().unwrap();
            }
            else {
                op = 0;
            }

            // Execute the command
            match command {
                "noop" => target = counter + 1,
                "addx" => {
                    target = counter + 2;
                }
                // Everything else
                _ => break,
            }
        }

        // reg_x < crt_pos <= reg_x+3
        if reg_x-1 <= (crt_pos % 40) && (crt_pos % 40) <= reg_x+1 {
            crt_array[(crt_pos) as usize] = '#';
        }
        else {
            crt_array[(crt_pos) as usize] = '.';
        }

        // Print the cycle, target, register, signal strength and command
        signal_strength = reg_x * counter;
        println!("Cycle: {}, Target: {}, CRT_Pos: {}, Pixel: {}, Register: {}, Signal Strength: {}, Command: {} {}", counter, target, crt_pos, crt_array[crt_pos as usize], reg_x, signal_strength, command, op);

        match counter {
            20 => signal_strengths.push(signal_strength),
            60 => signal_strengths.push(signal_strength),
            100 => signal_strengths.push(signal_strength),
            140 => signal_strengths.push(signal_strength),
            180 => signal_strengths.push(signal_strength),
            220 => signal_strengths.push(signal_strength),
            _ => (),
        }

        counter += 1;
        crt_pos = (counter - 1) % (40*6);

    }

    // Print the CRT
    for i in 0..6 {
        for j in 0..40 {
            print!("{}", crt_array[(i*40)+j]);
        }
        println!("");
    }

    // Print the signal strengths
    println!("Signal Strengths: {:?}", signal_strengths);

    // Sum the signal strengths
    let sum: i32 = signal_strengths.iter().sum();
    println!("Sum: {}", sum);
}
