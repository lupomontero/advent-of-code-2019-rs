use std::io::{self, Read};

// Part 1
fn run(buffer: &str, noun: u32, verb: u32) -> u32 {
    let v: Vec<&str> = buffer.trim().split(',').collect();
    let mut parsed: Vec<u32> = v.into_iter().map(|s| s.parse::<u32>().unwrap()).collect();
    let mut i = 0;

    parsed[1] = noun;
    parsed[2] = verb;

    loop {
        let op = parsed[i];

        if op == 99 {
            break;
        }

        let left = parsed[i + 1] as usize;
        let right = parsed[i + 2] as usize;
        let out = parsed[i + 3] as usize;

        match op {
            1 => {
                parsed[out] = parsed[left] + parsed[right];
            }
            2 => {
                parsed[out] = parsed[left] * parsed[right];
            }
            _ => panic!("Unknown operation: {}", op),
        }

        if i < parsed.len() - 4 {
            i += 4;
        } else {
            break;
        }
    }

    parsed[0]
}

// Part 2
fn find_inputs(buffer: &str, out: u32) -> u32 {
    let mut noun: i32 = -1;
    let mut verb: i32 = -1;

    for i in 0..100 {
        if noun >= 0 {
            break;
        }
        for j in 0..100 {
            if run(&buffer, i, j) == out {
                noun = i as i32;
                verb = j as i32;
                break;
            }
        }
    }

    if noun < 0 || verb < 0 {
        panic!("Noun or verb not found");
    }

    (100 * noun + verb) as u32
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", run(&buffer, 12, 2));
    println!("{:?}", find_inputs(&buffer, 19690720));
    Ok(())
}
