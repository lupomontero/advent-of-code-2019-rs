use std::io::{self, Read};

// Part 1
fn get_fuel_requirements_from_buffer(buffer: &str) -> u32 {
    let mut sum = 0;

    for line in buffer.lines() {
        let mass: u32 = line.parse().unwrap();
        let divided = mass as f32 / 3_f32;
        let rounded_down = divided.floor() as u32;
        sum += rounded_down - 2;
    }

    sum
}

// Part 2
fn get_fuel_requirements_from_mass(mass: u32) -> u32 {
    let divided = mass as f32 / 3_f32;
    let rounded_down = divided.floor() as i32;
    let fuel_required = rounded_down - 2;

    if fuel_required <= 0 {
        0
    } else {
        fuel_required as u32 + get_fuel_requirements_from_mass(fuel_required as u32)
    }
}

fn get_fuel_requirements_incl_fuel_mass_from_buffer(buffer: &str) -> u32 {
    let mut sum = 0;

    for line in buffer.lines() {
        let mass: u32 = line.parse().unwrap();
        sum += get_fuel_requirements_from_mass(mass);
    }

    sum
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", get_fuel_requirements_from_buffer(&buffer));
    println!(
        "{:?}",
        get_fuel_requirements_incl_fuel_mass_from_buffer(&buffer)
    );
    Ok(())
}
