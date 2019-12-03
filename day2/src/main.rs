use std::fs::File;
// use std::io;
use std::io::prelude::*;

fn main() {
    const EXIT: usize = 99;
    let mut f = File::open("./input.txt").unwrap();
    let mut str_buf = String::new();

    f.read_to_string(&mut str_buf);
    let mut numbers: Vec<usize> = str_buf.split(",").map(|s| s.trim().parse::<usize>().unwrap()).collect();
    let mut opcode: usize = 0;
    let mut index: usize = 0;

    numbers[1] = 12;
    numbers[2] = 2;
    while opcode != EXIT {
        opcode = numbers[index];
        match opcode {
            1 => {
                let result = numbers[index + 3];
                let arg_1 = numbers[numbers[index + 1]];
                let arg_2 = numbers[numbers[index + 2]];
                numbers[result] = arg_1 + arg_2;
            },
            2 => {
                let result = numbers[index + 3];
                let arg_1 = numbers[numbers[index + 1]];
                let arg_2 = numbers[numbers[index + 2]];
                numbers[result] = arg_1 * arg_2;
            },
            99 => (),
            _ => println!("Error: Exiting with unknown opcode"),
        }
        index += 4;
    }
    println!("Position 0: {}", numbers[0]);
}
