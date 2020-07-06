use std::io::{self, Read};

mod intcode_computer;
mod intcode_computer_v2;

//
// Heap's Algorithm
//
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
//
fn generate_permutations(arr: &mut [i32; 5]) -> std::vec::Vec<[i32; 5]> {
    let mut permutations = vec![];

    fn generate(n: usize, arr: &mut [i32; 5], permutations: &mut std::vec::Vec<[i32; 5]>) {
        if n == 1 {
            permutations.push(arr.to_owned());
        } else {
            generate(n - 1, arr, permutations);
            for i in 0..(n - 1) {
                if n % 2 == 0 {
                    arr.swap(i, n - 1);
                } else {
                    arr.swap(0, n - 1);
                }
                generate(n - 1, arr, permutations);
            }
        }
    }

    generate(arr.len(), arr, &mut permutations);

    permutations
}

fn get_max_output(program: &str) -> (i32, [i32; 5]) {
    let mut phase_settings = [0, 1, 2, 3, 4];
    let permutations = generate_permutations(&mut phase_settings);
    let mut max_output = 0;
    let mut max_output_combo = [0, 0, 0, 0, 0];

    for permutation in permutations {
        let mut prev = 0;
        for phase_setting in &permutation {
            let output = intcode_computer::run(program, vec![phase_setting.to_owned(), prev]);
            prev = output[0];
        }
        if prev > max_output {
            max_output = prev;
            max_output_combo = permutation;
        }
    }

    (max_output, max_output_combo)
}

fn get_max_output_with_feedback_loop(program: &str) -> (i32, [i32; 5]) {
    let mut phase_settings = [5, 6, 7, 8, 9];
    let permutations = generate_permutations(&mut phase_settings);
    let mut max_output = 0;
    let mut max_output_combo = [0, 0, 0, 0, 0];

    for permutation in permutations {
        let mut amplifiers: std::vec::Vec<intcode_computer_v2::IntcodeComputer> = permutation
            .iter()
            .map(|phase_setting| {
                let mut amp = intcode_computer_v2::IntcodeComputer::from_str(program);
                amp.input(phase_setting.to_owned());
                amp.run();
                amp
            })
            .collect();

        let mut prev = 0;

        'outer: loop {
            for (i, amp) in amplifiers.iter_mut().enumerate() {
                amp.input(prev);
                let out = amp.output();
                if out.is_none() {
                    panic!("Expected output!");
                }
                prev = out.unwrap();
                if i == phase_settings.len() - 1 && !amp.is_waiting_for_input {
                    break 'outer;
                }
            }
        }

        if prev > max_output {
            max_output = prev;
            max_output_combo = permutation;
        }
    }

    (max_output, max_output_combo)
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let (max_output, max_output_combo) = get_max_output(&buf);
    println!("{:?} {:?}", max_output, max_output_combo);

    let (max_output, max_output_combo) = get_max_output_with_feedback_loop(&buf);
    println!("{:?} {:?}", max_output, max_output_combo);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_permutations() {
        let mut arr = [0, 1, 2, 3, 4];
        assert_eq!(generate_permutations(&mut arr).len(), 120);
    }

    #[test]
    fn test_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let (max_output, max_output_combo) = get_max_output(program);
        assert_eq!(max_output, 43210);
        assert_eq!(max_output_combo, [4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let (max_output, max_output_combo) = get_max_output(program);
        assert_eq!(max_output, 54321);
        assert_eq!(max_output_combo, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let (max_output, max_output_combo) = get_max_output(program);
        assert_eq!(max_output, 65210);
        assert_eq!(max_output_combo, [1, 0, 4, 3, 2]);
    }

    //
    // Part 2
    //

    #[test]
    fn test_4() {
        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let (max_output, max_output_combo) = get_max_output_with_feedback_loop(program);
        assert_eq!(max_output, 139629729);
        assert_eq!(max_output_combo, [9, 8, 7, 6, 5]);
    }

    #[test]
    fn test_5() {
        let program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let (max_output, max_output_combo) = get_max_output_with_feedback_loop(program);
        assert_eq!(max_output, 18216);
        assert_eq!(max_output_combo, [9, 7, 8, 5, 6]);
    }
}
