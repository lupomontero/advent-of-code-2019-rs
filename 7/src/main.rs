use std::io::{self, Read};

mod intcode_computer;

//
// Heap's Algorithm
//
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
//
fn generate_permutations(arr: &mut [i32; 5]) -> std::vec::Vec<[i32; 5]> {
    let mut permutations = vec![];

    fn swap(arr: &mut [i32; 5], a: i32, b: i32) {
        let temp = arr[a as usize];
        arr[a as usize] = arr[b as usize];
        arr[b as usize] = temp;
    }

    fn generate(n: usize, arr: &mut [i32; 5], permutations: &mut std::vec::Vec<[i32; 5]>) {
        if n == 1 {
            permutations.push(arr.to_owned());
        } else {
            generate(n - 1, arr, permutations);
            for i in 0..(n - 1) {
                if n % 2 == 0 {
                    swap(arr, i as i32, n as i32 - 1);
                } else {
                    swap(arr, 0, n as i32 - 1);
                }
                generate(n - 1, arr, permutations);
            }
        }
    }

    generate(arr.len(), arr, &mut permutations);

    return permutations;
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

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let (max_output, max_output_combo) = get_max_output(&buf);
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
}