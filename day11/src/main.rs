
const ROUNDS: u128 = 10000;
const DIV3: bool = false;
const MOD: bool = true;
const MOD_OP: u128 = ((5*2*13*19*11*3*7*17) as u128).pow(2);

fn main() {
    let mut monkey0: Vec<u128> = vec![78, 53, 89, 51, 52, 59, 58, 85];
    let mut monkey1: Vec<u128> = vec![64];
    let mut monkey2: Vec<u128> = vec![71, 93, 65, 82];
    let mut monkey3: Vec<u128> = vec![67, 73, 95, 75, 56, 74];
    let mut monkey4: Vec<u128> = vec![85, 91, 90];
    let mut monkey5: Vec<u128> = vec![67, 96, 69, 55, 70, 83, 62];
    let mut monkey6: Vec<u128> = vec![53, 86, 98, 70, 64];
    let mut monkey7: Vec<u128> = vec![88, 64];
    let mut monkey0_inspect_count = 0;
    let mut monkey1_inspect_count = 0;
    let mut monkey2_inspect_count = 0;
    let mut monkey3_inspect_count = 0;
    let mut monkey4_inspect_count = 0;
    let mut monkey5_inspect_count = 0;
    let mut monkey6_inspect_count = 0;
    let mut monkey7_inspect_count = 0;

    for round in 1..=ROUNDS {
        println!("### Round {} ###", round);

        println!("Monkey 0: {:?}", monkey0);
        loop {
            if monkey0.len() == 0 {
                break;
            }

            monkey0 = monkey0_inspect(monkey0);
            monkey0_inspect_count += 1;

            let item = monkey0.remove(0);
            if item % 5 == 0 {
                monkey2.push(item);
                println!("    Item {} goes to Monkey 2", item);
            }
            else {
                monkey7.push(item);
                println!("    Item {} goes to Monkey 7", item);
            }
        }

        println!("Monkey 1: {:?}", monkey1);
        loop {
            if monkey1.len() == 0 {
                break;
            }
            monkey1 = monkey1_inspect(monkey1);
            monkey1_inspect_count += 1;

            let item = monkey1.remove(0);
            if item % 2 == 0 {
                monkey3.push(item);
                println!("    Item {} goes to Monkey 3", item);
            }
            else {
                monkey6.push(item);
                println!("    Item {} goes to Monkey 6", item);
            }
        }

        println!("Monkey 2: {:?}", monkey2);
        loop {
            if monkey2.len() == 0 {
                break;
            }
            monkey2 = monkey2_inspect(monkey2);
            monkey2_inspect_count += 1;

            let item = monkey2.remove(0);
            if item % 13 == 0 {
                monkey5.push(item);
                println!("    Item {} goes to Monkey 5", item);
            }
            else {
                monkey4.push(item);
                println!("    Item {} goes to Monkey 4", item);
            }
        }

        println!("Monkey 3: {:?}", monkey3);
        loop {
            if monkey3.len() == 0 {
                break;
            }
            monkey3 = monkey3_inspect(monkey3);
            monkey3_inspect_count += 1;

            let item = monkey3.remove(0);
            if item % 19 == 0 {
                monkey6.push(item);
                println!("    Item {} goes to Monkey 6", item);
            }
            else {
                monkey0.push(item);
                println!("    Item {} goes to Monkey 0", item);
            }
        }

        println!("Monkey 4: {:?}", monkey4);
        loop {
            if monkey4.len() == 0 {
                break;
            }
            monkey4 = monkey4_inspect(monkey4);
            monkey4_inspect_count += 1;

            let item = monkey4.remove(0);
            if item % 11 == 0 {
                monkey3.push(item);
                println!("    Item {} goes to Monkey 3", item);
            }
            else {
                monkey1.push(item);
                println!("    Item {} goes to Monkey 1", item);
            }
        }

        println!("Monkey 5: {:?}", monkey5);
        loop {
            if monkey5.len() == 0 {
                break;
            }
            monkey5 = monkey5_inspect(monkey5);
            monkey5_inspect_count += 1;

            let item = monkey5.remove(0);
            if item % 3 == 0 {
                monkey4.push(item);
                println!("    Item {} goes to Monkey 4", item);
            }
            else {
                monkey1.push(item);
                println!("    Item {} goes to Monkey 1", item);
            }
        }

        println!("Monkey 6: {:?}", monkey6);
        loop {
            if monkey6.len() == 0 {
                break;
            }
            monkey6 = monkey6_inspect(monkey6);
            monkey6_inspect_count += 1;

            let item = monkey6.remove(0);
            if item % 7 == 0 {
                monkey7.push(item);
                println!("    Item {} goes to Monkey 7", item);
            }
            else {
                monkey0.push(item);
                println!("    Item {} goes to Monkey 0", item);
            }
        }

        println!("Monkey 7: {:?}", monkey7);
        loop {
            if monkey7.len() == 0 {
                break;
            }
            monkey7 = monkey7_inspect(monkey7);
            monkey7_inspect_count += 1;

            let item = monkey7.remove(0);
            if item % 17 == 0 {
                monkey2.push(item);
                println!("    Item {} goes to Monkey 2", item);
            }
            else {
                monkey5.push(item);
                println!("    Item {} goes to Monkey 5", item);
            }
        }
    }

    // Print the inspect counts
    println!("Monkey 0 inspect count: {}", monkey0_inspect_count);
    println!("Monkey 1 inspect count: {}", monkey1_inspect_count);
    println!("Monkey 2 inspect count: {}", monkey2_inspect_count);
    println!("Monkey 3 inspect count: {}", monkey3_inspect_count);
    println!("Monkey 4 inspect count: {}", monkey4_inspect_count);
    println!("Monkey 5 inspect count: {}", monkey5_inspect_count);
    println!("Monkey 6 inspect count: {}", monkey6_inspect_count);
    println!("Monkey 7 inspect count: {}", monkey7_inspect_count);

    // Find the max two inspect counts and multiply them together
    let mut inspect_counts: Vec<u128> = vec![monkey0_inspect_count, monkey1_inspect_count, monkey2_inspect_count, monkey3_inspect_count, monkey4_inspect_count, monkey5_inspect_count, monkey6_inspect_count, monkey7_inspect_count];
    inspect_counts.sort();
    let total_inspect_counts : u128 = (inspect_counts[inspect_counts.len()-1]) as u128 * (inspect_counts[inspect_counts.len()-2]) as u128;
    println!("Max two inspect counts multiplied: {}", total_inspect_counts);
}

fn monkey0_inspect(mut items: Vec<u128>) -> Vec<u128> {
    let mut item = items[0]*3;
    item = if DIV3 {item/3} else {item};
    item = if MOD {item % MOD_OP} else {item};
    println!("  Monkey 0 inspects {} and it becomes {}", items[0], item);
    items[0] = item;
    items
}
fn monkey1_inspect(mut items: Vec<u128>) -> Vec<u128> {
    let mut item = items[0]+7;
    item = if DIV3 {item/3} else {item};
    item = if MOD {item % MOD_OP} else {item};
    println!("  Monkey 1 inspects {} and it becomes {}", items[0], item);
    items[0] = item;
    items
}
fn monkey2_inspect(mut items: Vec<u128>) -> Vec<u128> {
    let mut item = items[0]+5;
    item = if DIV3 {item/3} else {item};
    item = if MOD {item % MOD_OP} else {item};
    println!("  Monkey 2 inspects {} and it becomes {}", items[0], item);
    items[0] = item;
    items
}
fn monkey3_inspect(mut items: Vec<u128>) -> Vec<u128> {
    let mut item = items[0]+8;
    item = if DIV3 {item/3} else {item};
    item = if MOD {item % MOD_OP} else {item};
    println!("  Monkey 3 inspects {} and it becomes {}", items[0], item);
    items[0] = item;
    items
}
fn monkey4_inspect(mut items: Vec<u128>) -> Vec<u128> {
    let mut item = items[0]+4;
    item = if DIV3 {item/3} else {item};
    item = if MOD {item % MOD_OP} else {item};
    println!("  Monkey 4 inspects {} and it becomes {}", items[0], item);
    items[0] = item;
    items
}
fn monkey5_inspect(mut items: Vec<u128>) -> Vec<u128> {
    let mut item = items[0]*2;
    item = if DIV3 {item/3} else {item};
    item = if MOD {item % MOD_OP} else {item};
    println!("  Monkey 5 inspects {} and it becomes {}", items[0], item);
    items[0] = item;
    items
}
fn monkey6_inspect(mut items: Vec<u128>) -> Vec<u128> {
    let mut item = items[0]+6;
    item = if DIV3 {item/3} else {item};
    item = if MOD {item % MOD_OP} else {item};
    println!("  Monkey 6 inspects {} and it becomes {}", items[0], item);
    items[0] = item;
    items
}
fn monkey7_inspect(mut items: Vec<u128>) -> Vec<u128> {
    let mut item = items[0]*items[0];
    item = if DIV3 {item/3} else {item};
    item = if MOD {item % MOD_OP} else {item};
    println!("  Monkey 7 inspects {} and it becomes {}", items[0], item);
    items[0] = item;
    items
}