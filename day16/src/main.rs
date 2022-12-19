extern crate pathfinding;

use pathfinding::prelude::dijkstra;

static raw_input: &str = include_str!("input_formatted.txt");

const TIME_REMAINING: i32 = 30;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node<'a> {
    name: &'a str,
    rate: i32,
    tunnels: Vec<&'a str>,
}
impl Node<'_> {
    fn successors<'a>(&self, nodes: &'a Vec<Node<'a>>) -> Vec<(&'a Node<'a>, i32)> {
        let difficulty = 1;
        let mut successors: Vec<(&Node, i32)> = Vec::new();
        for tunnel in &self.tunnels {
            let node = find_node(nodes, tunnel).unwrap();
            successors.push((node, difficulty));
        }
        successors
    }
}

fn main() {
    // Parse the raw_input into a vector of lines
    let lines: Vec<&str> = raw_input.lines().collect();

    // Create a vector of nodes
    let mut nodes: Vec<Node> = Vec::new();

    // Iterate over the lines
    for line in lines {
        // Split the line into a vector of words
        let words: Vec<&str> = line.split_whitespace().collect();

        // Create a vec of strings from the 2nd word onwards
        let mut tunnels: Vec<&str> = Vec::new();
        for i in 2..words.len() {
            tunnels.push(words[i]);
        }

        // Create a new node
        let node = Node {
            name: words[0],
            rate: words[1].parse().unwrap(),
            tunnels: tunnels,
        };

        // Add the node to the vector
        nodes.push(node);
    }

    let mut steam_released = 0;
    let mut current_node = find_node(&nodes, "AA").unwrap();
    let mut turned_on: Vec<&Node> = Vec::new();
    for minute in 1..=TIME_REMAINING {
        // Print the current state
        println!("### Current Node: {} ###", current_node.name);
        println!("Minute: {}", minute);
        println!("Steam Released: {}", steam_released);
        println!("Turned On: {:?}", turned_on);

        // Calculate the total steam released every turn
        for node in &turned_on {
            steam_released += node.rate;
        }

        // Create vector of nodes total steam values
        let mut max_steams: Vec<(&str, i32, i32, Vec<&Node>)> = Vec::new();
        for node in &nodes {
            let path = dijkstra(
                &current_node,
                |n| n.successors(&nodes),
                |n| n.name == node.name,
            )
            .unwrap();
            let distance = path.1;

            // Calculate the total steam, unless its already on then 0
            let total_steam = if turned_on.contains(&node) {
                0
            } else {
                calc_total_steam(node, distance, minute)
            };

            max_steams.push((node.name, total_steam, distance, path.0));
        }

        // Sort the vector by total steam
        max_steams.sort_by(|a, b| b.1.cmp(&a.1));

        // If the path distance is not 1, then we have to move to the next node
        // Set the current node to the next node
        if max_steams[0].2 != 0 {
            current_node = max_steams[0].3[1];
        }
        else {
            // Set the current node as turned on
            turned_on.push(current_node);
        }

        // Print the target and it's total steam
        println!("{}'s Steam: {}", max_steams[0].0, max_steams[0].1);
        // Print path to next node as AA -> BB -> CC
        let mut path_string = String::new();
        for node in &max_steams[0].3 {
            path_string.push_str(node.name);
            path_string.push_str(" -> ");
        }
        println!("Path: {}", path_string);
    }
}

// Add lifetime to the function
fn find_node<'a>(nodes: &'a Vec<Node>, name: &str) -> Option<&'a Node<'a>> {
    for node in nodes {
        if node.name == name {
            return Some(node);
        }
    }
    None
}

fn calc_total_steam(node: &Node, distance: i32, minute: i32) -> i32 {
    let mut time_remaining = TIME_REMAINING - minute;

    let mut total_steam = (time_remaining - distance) * node.rate;
    total_steam
}
