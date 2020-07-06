#[derive(Debug)]
pub struct IntcodeComputer {
    memory: std::vec::Vec<i32>,
    program_pointer: usize,
    input_queue: std::vec::Vec<i32>,
    output_queue: std::vec::Vec<i32>,
    pub is_waiting_for_input: bool,
    has_ended: bool,
}

impl IntcodeComputer {
    pub fn from_str(buf: &str) -> Self {
        let v: Vec<&str> = buf.trim().split(',').collect();
        let memory: Vec<i32> = v.into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
        Self {
            memory,
            program_pointer: 0,
            input_queue: vec![],
            output_queue: vec![],
            is_waiting_for_input: false,
            has_ended: false,
        }
    }

    fn process_instruction(&mut self) {
        let operation = self.memory[self.program_pointer] % 100;
        let param_modes = (self.memory[self.program_pointer] / 100).to_string();
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
            let val = self.memory[self.program_pointer + 1 + j];
            if mode == '0' && *mode_setting == 1 {
                params.push(self.memory[val as usize]);
            } else {
                params.push(val);
            }
        }

        if operation == 1 {
            self.memory[params[2] as usize] = params[0] + params[1];
        } else if operation == 2 {
            self.memory[params[2] as usize] = params[0] * params[1];
        } else if operation == 3 {
            if self.input_queue.is_empty() {
                self.is_waiting_for_input = true;
                return;
            } else {
                self.memory[params[0] as usize] = self.input_queue.remove(0);
            }
        } else if operation == 4 {
            self.output_queue.push(params[0]);
        } else if operation == 5 && params[0] != 0 || operation == 6 && params[0] == 0 {
            self.program_pointer = params[1] as usize;
            return;
        } else if operation == 7 {
            self.memory[params[2] as usize] = if params[0] < params[1] { 1 } else { 0 };
        } else if operation == 8 {
            self.memory[params[2] as usize] = if params[0] == params[1] { 1 } else { 0 };
        } else if operation == 99 {
            self.has_ended = true;
        }

        self.program_pointer = self.program_pointer + params.len() + 1;
    }

    pub fn run(&mut self) {
        loop {
            if self.program_pointer >= self.memory.len()
                || self.has_ended
                || self.is_waiting_for_input
            {
                break;
            }
            self.process_instruction();
        }
    }

    pub fn input(&mut self, value: i32) {
        self.input_queue.push(value);
        if self.is_waiting_for_input {
            self.is_waiting_for_input = false;
            self.run();
        }
    }

    pub fn output(&mut self) -> Option<i32> {
        if self.output_queue.is_empty() {
            None
        } else {
            Some(self.output_queue.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_computer() {
        let mut computer = IntcodeComputer::from_str("3,0,4,0,99");
        computer.run();
        computer.input(1);
        assert_eq!(computer.output(), Some(1));
        assert_eq!(computer.output(), None);
    }

    #[test]
    fn test_run_2() {
        let mut computer = IntcodeComputer::from_str("3,9,8,9,10,9,4,9,99,-1,8");
        computer.run();
        computer.input(1);
        assert_eq!(computer.output(), Some(0));
        assert_eq!(computer.output(), None);
    }

    #[test]
    fn test_run_3() {
        let mut computer = IntcodeComputer::from_str("3,9,7,9,10,9,4,9,99,-1,8");
        computer.input(1);
        computer.run();
        assert_eq!(computer.output(), Some(1));
        assert_eq!(computer.output(), None);
    }

    // #[test]
    // fn test_run_4() {
    //     assert_eq!(run("3,3,1108,-1,8,3,4,3,99", vec![8]), vec![1]);
    // }

    // #[test]
    // fn test_run_5() {
    //     assert_eq!(run("3,3,1107,-1,8,3,4,3,99", vec![7]), vec![1]);
    // }

    // #[test]
    // fn test_run_6() {
    //     assert_eq!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![2]), vec![1]);
    // }

    // #[test]
    // fn test_run_7() {
    //     assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![2]), vec![1]);
    // }

    // #[test]
    // fn test_run_8() {
    //     /*
    //     The below program uses an input instruction to ask for a single number.
    //     The program will then output `999` if the input value is below `8`, output
    //     `1000` if the input value is equal to `8`, or output `1001` if the input
    //     value is greater than `8`.
    //     */
    //     let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    //     assert_eq!(run(program, vec![0]), vec![999]);
    //     assert_eq!(run(program, vec![8]), vec![1000]);
    //     assert_eq!(run(program, vec![9]), vec![1001]);
    // }

    // #[test]
    // fn test_run_with_multiple_inputs() {
    //   let program = "3,9,3,10,4,9,4,10,99,0,0";
    //   assert_eq!(run(program, vec![6, 3]), vec![6, 3]);
    //   assert_eq!(run(program, vec![123, 456]), vec![123, 456]);
    // }
}
