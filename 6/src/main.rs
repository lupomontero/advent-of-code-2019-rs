use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct Node {
    id: String,
    parent: String,
}

fn build_hash(buffer: &str) -> HashMap<&str, Node> {
    let mut hash = HashMap::new();

    for (i, line) in buffer.lines().enumerate() {
        let parts = line.split(')').collect::<Vec<&str>>();

        hash.entry(parts[0]).or_insert(Node {
            id: parts[0].to_owned(),
            parent: "".to_owned(),
        });

        hash.insert(
            parts[1],
            Node {
                id: parts[1].to_owned(),
                parent: parts[0].to_owned(),
            },
        );
    }

    hash
}

fn get_steps(hash: &HashMap<&str, Node>, node: &Node) -> u32 {
    if node.parent == "" {
        0
    } else {
        1 + get_steps(&hash, hash.get(&node.parent[..]).unwrap())
    }
}

fn get_total_orbits(hash: &HashMap<&str, Node>) -> u32 {
    let mut count = 0;
    for node in hash.values() {
        count += get_steps(&hash, &node)
    }
    count
}

// Part 2
fn get_ancestors(hash: &HashMap<&str, Node>, node: &Node) -> Vec<String> {
    let mut ancestors = vec![node.id.clone()];

    if let Some(parent) = hash.get(&node.parent[..]) {
        ancestors.append(&mut get_ancestors(hash, parent));
    }

    ancestors
}

fn get_min_orbital_transfers(hash: &HashMap<&str, Node>, from: &str, to: &str) -> u32 {
    let from_node = hash.get(from).unwrap();
    let to_node = hash.get(to).unwrap();
    let from_ancestors = get_ancestors(&hash, &from_node);
    let to_ancestors = get_ancestors(&hash, &to_node);

    for (i, from_ancestor) in from_ancestors.iter().enumerate() {
        for (j, to_ancestor) in to_ancestors.iter().enumerate() {
            if from_ancestor == to_ancestor {
                return (i + j - 2) as u32;
            }
        }
    }

    panic!("No common ancestors!")
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let hash = build_hash(&buffer);
    println!("{:?}", get_total_orbits(&hash));
    println!("{:?}", get_min_orbital_transfers(&hash, "YOU", "SAN"));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn test_get_total_orbits() {
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
        let hash = build_hash(&buffer);
        assert_eq!(get_total_orbits(&hash), 42);
    }

    // Part 2
    #[test]
    fn test_get_ancestors() {
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
K)YOU
I)SAN";
        let hash = build_hash(&buffer);
        let node = hash.get("YOU").unwrap();
        assert_eq!(
            get_ancestors(&hash, node),
            vec!["YOU", "K", "J", "E", "D", "C", "B", "COM"]
        );
    }

        #[test]
        fn test_get_min_orbital_transfers() {
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
K)YOU
I)SAN";
            let hash = build_hash(&buffer);
            assert_eq!(get_min_orbital_transfers(&hash, "YOU", "SAN"), 4);
        }
}
