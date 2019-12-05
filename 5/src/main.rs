use std::io::{self, Read};

fn process_instruction(v: &mut Vec<i32>, i: usize, input: i32) -> (i32, usize) {
    let operation = v[i] % 100;
    let param_modes = (v[i] / 100).to_string();
    let mut param_modes = param_modes.chars().rev();
    let params_modes_settings = match operation {
        1 => vec![1, 1, 0],
        2 => vec![1, 1, 0],
        3 => vec![0],
        4 => vec![1],
        5 => vec![1, 1],
        6 => vec![1, 1],
        7 => vec![1, 1, 0],
        8 => vec![1, 1, 0],
        99 => vec![],
        _ => panic!("Unsupported operation: {}", operation),
    };

    let mut params = vec![];

    for (j, mode_setting) in params_modes_settings.iter().enumerate() {
        let mode = match param_modes.next() {
            Some(c) => c,
            None => '0',
        };
        let val = v[i + 1 + j];
        if mode == '0' && *mode_setting == 1 {
            params.push(v[val as usize]);
        } else {
            params.push(val);
        }
    }

    let mut next = i + params.len() + 1;

    if operation == 1 {
        v[params[2] as usize] = params[0] + params[1];
    } else if operation == 2 {
        v[params[2] as usize] = params[0] * params[1];
    } else if operation == 3 {
        v[params[0] as usize] = input;
    } else if operation == 4 {
        println!("OUTPUT: {:?}", params[0]);
    } else if operation == 5 && params[0] != 0 || operation == 6 && params[0] == 0 {
        next = params[1] as usize;
    } else if operation == 7 {
        v[params[2] as usize] = if params[0] < params[1] { 1 } else { 0 };
    } else if operation == 8 {
        v[params[2] as usize] = if params[0] == params[1] { 1 } else { 0 };
    }

    (operation, next)
}

fn run(buffer: &str, input: i32) -> i32 {
    let v: Vec<&str> = buffer.trim().split(',').collect();
    let mut parsed: Vec<i32> = v.into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut i = 0;

    loop {
        if i >= parsed.len() {
            break;
        }

        let (operation, next) = process_instruction(&mut parsed, i, input);

        if operation == 99 {
            break;
        }

        i = next;
    }

    parsed[0]
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", run(&buffer, 5));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1

    #[test]
    fn test_instruction_with_default_param_modes() {
        let mut program = vec![1, 0, 0, 3];
        let (operation, next) = process_instruction(&mut program, 0, 1);
        assert_eq!(operation, 1);
        assert_eq!(next, 4);
        assert_eq!(program, vec![1, 0, 0, 2]);
    }

    #[test]
    fn test_instruction_with_mixed_modes() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let (operation, next) = process_instruction(&mut program, 0, 1);
        assert_eq!(operation, 2);
        assert_eq!(next, 4);
        assert_eq!(program, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_run_1() {
        // TODO: test that it actually outputs 1
        assert_eq!(run("3,0,4,0,99", 1), 1);
    }

    #[test]
    fn test_run_2() {
        assert_eq!(run("1002,4,3,4,33", 1), 1002);
    }

    #[test]
    fn test_run_3() {
        assert_eq!(run("1101,100,-1,4,0", 1), 1101);
    }

    // // Part 2

    #[test]
    fn test_run_4() {
        // TODO: test that it actually outputs 0
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", 1), 3);
    }

    #[test]
    fn test_run_5() {
        // TODO: test that it actually outputs 1
        assert_eq!(run("3,9,7,9,10,9,4,9,99,-1,8", 1), 3);
    }

    #[test]
    fn test_run_6() {
        // TODO: test that it actually outputs 1
        assert_eq!(run("3,3,1108,-1,8,3,4,3,99", 8), 3);
    }

    #[test]
    fn test_run_7() {
        // TODO: test that it actually outputs 1
        assert_eq!(run("3,3,1107,-1,8,3,4,3,99", 7), 3);
    }

    #[test]
    fn test_run_8() {
        // TODO: test that it actually outputs 1 (input was non zero)
        assert_eq!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 2), 3);
    }

    #[test]
    fn test_run_9() {
        // TODO: test that it actually outputs 1 (input was non zero)
        assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 2), 3);
    }

    #[test]
    fn test_run_10() {
        // TODO: test that it actually outputs 1 (input was non zero)
        assert_eq!(run("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 0), 3);
    }
}
