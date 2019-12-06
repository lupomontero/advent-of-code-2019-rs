use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    id: String,
    parent: String,
}

fn get_steps(hash: &HashMap<&str, Node>, node: &Node) -> u32 {
    if node.parent == "" {
        0
    } else {
        1 + get_steps(&hash, hash.get(&node.parent[..]).unwrap())
    }
}

fn build_hash(buffer: &str) -> HashMap<&str, Node> {
    let mut hash = HashMap::new();

    for (i, line) in buffer.lines().enumerate() {
        let parts = line.split(')').collect::<Vec<&str>>();

        hash.entry(parts[0]).or_insert(Node {
            id: parts[0].to_owned(),
            parent: "".to_owned(),
        });

        hash.insert(parts[1], Node {
            id: parts[1].to_owned(),
            parent: parts[0].to_owned(),
        });
    }

    hash
}

fn get_total_orbits(buffer: &str) -> u32 {
    let hash = build_hash(buffer);
    let mut count = 0;
    for node in hash.values() {
        count += get_steps(&hash, &node)
    }
    count
}

// Part 2
// fn do_something_else(buffer: &str) -> u32 {
//     0
// }

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", get_total_orbits(&buffer));
    // println!("{:?}", do_something_else(&buffer));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn test_do_something() {
        let buffer = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
        assert_eq!(get_total_orbits(&buffer), 42);
    }

    // Part 2
    // #[test]
    // fn test_do_something_else() {
    //     assert_eq!(do_something_else(""), 0);
    // }
}
