use std::io::{self, Read};

fn get_path_points(s: &str) -> Vec<(i32, i32)> {
    let v: Vec<&str> = s.trim().split(',').collect();
    let mut points = vec![(0, 0)];

    for item in v {
        let direction = item.chars().nth(0).unwrap();
        let amount = (&item[1..]).parse::<i32>().unwrap();

        for _ in 1..=amount {
            let prev = points[points.len() - 1];
            match direction {
                'U' => points.push((prev.0, prev.1 + 1)),
                'D' => points.push((prev.0, prev.1 - 1)),
                'L' => points.push((prev.0 - 1, prev.1)),
                'R' => points.push((prev.0 + 1, prev.1)),
                _ => panic!("Unexpected direction!"),
            }
        }
    }

    points
}

// Part 1
fn get_closest_intersection_distance(a: &str, b: &str) -> u32 {
    let a_points = get_path_points(a);
    let b_points = get_path_points(b);
    let mut closest_intersection_distance = -1;

    for point in a_points {
        if b_points.contains(&point) {
            let distance = point.0.abs() + point.1.abs();
            if distance == 0 {
                continue;
            }
            if closest_intersection_distance < 0 || distance < closest_intersection_distance {
                closest_intersection_distance = distance;
            }
        }
    }

    closest_intersection_distance as u32
}

// Part 2
fn get_intersection_with_fewer_steps(a: &str, b: &str) -> u32 {
    let a_points = get_path_points(a);
    let b_points = get_path_points(b);

    for (i, a_point) in a_points.iter().enumerate() {
        for (j, b_point) in b_points.iter().enumerate() {
            if i > 0 && j > 0 && a_point == b_point {
                return (i + j) as u32;
            }
        }
    }

    panic!("Paths do not intersect!");
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();
    let a = lines.next().unwrap();
    let b = lines.next().unwrap();

    println!("{}", get_closest_intersection_distance(a, b));
    println!("{}", get_intersection_with_fewer_steps(a, b));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_closest_intersection_distance_1() {
        assert_eq!(
            get_closest_intersection_distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
    }

    #[test]
    fn test_get_closest_intersection_distance_2() {
        assert_eq!(
            get_closest_intersection_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    // Part 2
    #[test]
    fn test_get_intersection_with_fewer_steps_1() {
        assert_eq!(
            get_intersection_with_fewer_steps(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
    }

    #[test]
    fn test_get_intersection_with_fewer_steps_2() {
        assert_eq!(
            get_intersection_with_fewer_steps(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
