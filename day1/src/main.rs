use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
// use std::str;

fn main() {
    let f = File::open("./input.txt").unwrap();
    let line_buf = BufReader::new(&f);
    let mut fuel: i64 = 0;

    for line in line_buf.lines() {
        let module_mass = line.ok().and_then(|l| l.parse::<i64>().ok()).unwrap_or(0);
        // let module_fuel = (module_mass / 3) - 2;
        fuel += module_fuel(module_mass);
    }

    println!("TOTAL FUEL: {}", fuel);
}

fn module_fuel(mass: i64) -> i64 {
    let mut fuel = (mass / 3) - 2;
    if fuel > 0 {
        fuel += module_fuel(fuel);
    } else {
        fuel = 0;
    }

    return fuel;
}
