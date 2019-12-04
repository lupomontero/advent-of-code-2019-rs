fn meets_criteria(n: u32) -> bool {
    let s = n.to_string();
    let mut has_repeating_digits = false;
    let mut has_decreasing_digits = false;

    for (i, c) in s.chars().enumerate() {
        if i > 0 {
            let prev = s.chars().nth(i - 1).unwrap();
            if c == prev {
                has_repeating_digits = true;
            }
            if c.to_digit(10).unwrap() < prev.to_digit(10).unwrap() {
                has_decreasing_digits = true;
            }
        }
    }

    has_repeating_digits && !has_decreasing_digits
}

fn meets_criteria_with_extra_criterion(n: u32) -> bool {
    let s = n.to_string();
    let mut chars = s.chars();
    let mut repeat_count = 0;
    let mut prev = chars.next().unwrap();
    let mut has_repeating_digits = false;
    let mut has_decreasing_digits = false;

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => {
                if repeat_count == 1 {
                    has_repeating_digits = true;
                }
                break;
            }
        };

        if c == prev {
            repeat_count += 1;
        } else {
            if repeat_count == 1 {
                has_repeating_digits = true;
            }
            repeat_count = 0;
            if c < prev {
                has_decreasing_digits = true;
            }
        }

        prev = c;
    }

    has_repeating_digits && !has_decreasing_digits
}

// Part 1
// Argument `extra_criterion` added for part 2
fn count_possible_passwords(min: u32, max: u32, extra_criterion: bool) -> u32 {
    let mut count = 0;

    for n in min..max {
        if !extra_criterion && meets_criteria(n) {
            count += 1;
        } else if extra_criterion && meets_criteria_with_extra_criterion(n) {
            count += 1;
        }
    }

    count
}

fn main() {
    let min = 145852;
    let max = 616942;
    println!("{:?}", count_possible_passwords(min, max, false));
    println!("{:?}", count_possible_passwords(min, max, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn test_meets_criteria_1() {
        assert_eq!(meets_criteria(111111), true);
    }

    #[test]
    fn test_meets_criteria_2() {
        assert_eq!(meets_criteria(223450), false);
    }

    #[test]
    fn test_meets_criteria_3() {
        assert_eq!(meets_criteria(123789), false);
    }

    // Part 2
    #[test]
    fn test_meets_criteria_with_extra_criterion_1() {
        assert_eq!(meets_criteria_with_extra_criterion(112233), true);
    }

    #[test]
    fn test_meets_criteria_with_extra_criterion_2() {
        assert_eq!(meets_criteria_with_extra_criterion(123444), false);
    }

    #[test]
    fn test_meets_criteria_with_extra_criterion_3() {
        assert_eq!(meets_criteria_with_extra_criterion(111122), true);
    }

    #[test]
    fn test_meets_criteria_with_extra_criterion_4() {
        assert_eq!(meets_criteria_with_extra_criterion(1111222), false);
    }
}
