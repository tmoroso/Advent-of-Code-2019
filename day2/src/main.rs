use std::fs::File;
use std::process::exit;
use std::io::prelude::*;

fn execute_program(noun: usize, verb: usize) -> usize {
    const EXIT: usize = 99;
    let mut f = File::open("./input.txt").unwrap();
    let mut str_buf = String::new();

    f.read_to_string(&mut str_buf);
    let mut numbers: Vec<usize> = str_buf.split(",").map(|s| s.trim().parse::<usize>().unwrap()).collect();
    let mut opcode: usize = 0;
    let mut index: usize = 0;

    numbers[1] = noun;
    numbers[2] = verb;
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

    return numbers[0];
}

fn main() {
    const DESIRED_OUT: usize = 19690720;

    for n in 0..100 {
        for v in 0..100 {
            let output = execute_program(n, v);
            if output == DESIRED_OUT {
                println!("100 * noun + verb = {}", (100*n) + v);
                exit(0);
            }
        }
    }
}
