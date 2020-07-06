fn process_instruction(
    v: &mut Vec<i32>,
    i: usize,
    input: &mut std::vec::Vec<i32>,
) -> (i32, Option<i32>, usize) {
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

    let mut output: Option<i32> = None;
    let mut next = i + params.len() + 1;

    if operation == 1 {
        v[params[2] as usize] = params[0] + params[1];
    } else if operation == 2 {
        v[params[2] as usize] = params[0] * params[1];
    } else if operation == 3 {
        v[params[0] as usize] = input.remove(0);
    } else if operation == 4 {
        output = Some(params[0]);
    } else if operation == 5 && params[0] != 0 || operation == 6 && params[0] == 0 {
        next = params[1] as usize;
    } else if operation == 7 {
        v[params[2] as usize] = if params[0] < params[1] { 1 } else { 0 };
    } else if operation == 8 {
        v[params[2] as usize] = if params[0] == params[1] { 1 } else { 0 };
    }

    (operation, output, next)
}

pub fn run(buffer: &str, input: std::vec::Vec<i32>) -> std::vec::Vec<i32> {
    let v: Vec<&str> = buffer.trim().split(',').collect();
    let mut parsed: Vec<i32> = v.into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut i = 0;
    let mut outputs = vec![];
    let mut input = input;

    loop {
        if i >= parsed.len() {
            break;
        }

        let (operation, output, next) = process_instruction(&mut parsed, i, &mut input);
        if let Some(output) = output {
            outputs.push(output);
        }

        if operation == 99 {
            break;
        }

        i = next;
    }

    outputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = vec![1];
        let output = run("3,0,4,0,99", input);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn test_run_2() {
        let input = vec![1];
        let output = run("3,9,8,9,10,9,4,9,99,-1,8", input);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn test_run_3() {
        let input = vec![1];
        let output = run("3,9,7,9,10,9,4,9,99,-1,8", input);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn test_run_4() {
        assert_eq!(run("3,3,1108,-1,8,3,4,3,99", vec![8]), vec![1]);
    }

    #[test]
    fn test_run_5() {
        assert_eq!(run("3,3,1107,-1,8,3,4,3,99", vec![7]), vec![1]);
    }

    #[test]
    fn test_run_6() {
        assert_eq!(
            run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![2]),
            vec![1]
        );
    }

    #[test]
    fn test_run_7() {
        assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![2]), vec![1]);
    }

    #[test]
    fn test_run_8() {
        /*
        The below program uses an input instruction to ask for a single number.
        The program will then output `999` if the input value is below `8`, output
        `1000` if the input value is equal to `8`, or output `1001` if the input
        value is greater than `8`.
        */
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run(program, vec![0]), vec![999]);
        assert_eq!(run(program, vec![8]), vec![1000]);
        assert_eq!(run(program, vec![9]), vec![1001]);
    }

    #[test]
    fn test_run_with_multiple_inputs() {
        let program = "3,9,3,10,4,9,4,10,99,0,0";
        assert_eq!(run(program, vec![6, 3]), vec![6, 3]);
        assert_eq!(run(program, vec![123, 456]), vec![123, 456]);
    }
}
