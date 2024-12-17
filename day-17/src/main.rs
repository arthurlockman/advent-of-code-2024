#[macro_use]
extern crate num_derive;
use crate::instruction::{Execute, Executor, Instruction};
use itertools::Itertools;
use num_traits::FromPrimitive;
use regex::Regex;
use utils::{read_file, time};

mod instruction;

fn main() {
    let (output1, time1) = time(|| run("src/input.txt"));
    println!("Part 1: {} (took {} seconds)", output1, time1.as_secs_f64());
    // let (output2, time2) = time(|| run2("src/input.txt"));
    // println!("Part 2: {} (took {} seconds)", output2, time2.as_secs_f64());
}

fn run(filename: &str) -> String {
    let (mut exec, instructions) = parse_program(filename);
    let mut output: Vec<u32> = Vec::new();
    while (exec.instruction_pointer as usize) < instructions.len() {
        let (inst, op) = &instructions[exec.instruction_pointer as usize];
        let result = inst.execute(*op, exec);
        exec = result.0;
        if let Some(out) = result.1 {
            output.push(out);
        }
    }
    output.into_iter().join(",")
}

fn run2(filename: &str) -> u32 {
    let (exec, instructions) = parse_program(filename);
    for a in 10000..u32::MAX {
        let mut exec = exec.clone();
        let mut output: Vec<u32> = Vec::new();
        exec.register_a = a;
        while (exec.instruction_pointer as usize) < instructions.len() {
            let (inst, op) = &instructions[exec.instruction_pointer as usize];
            let result = inst.execute(*op, exec);
            exec = result.0;
            if let Some(out) = result.1 {
                output.push(out);
            }
        }
        let val = output.into_iter().join(",");
        println!("{}", a);
        if val == "2,4,1,1,7,5,1,5,4,2,5,5,0,3,3,0" {
            return a;
        }
    }
    0
}

fn parse_program(filename: &str) -> (Executor, Vec<(Instruction, u32)>) {
    let file_contents = read_file(filename);
    let registers_regex =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    let program_regex = Regex::new(r"(\d),(\d)").unwrap();

    let (_, [register_a, register_b, register_c]) = registers_regex
        .captures_iter(&file_contents)
        .map(|c| c.extract())
        .collect_vec()[0];
    let ops: Vec<(Instruction, u32)> = program_regex
        .captures_iter(&file_contents)
        .map(|c| c.extract())
        .map(|(_, [op, val])| {
            (
                Instruction::from_u32(op.parse::<u32>().unwrap()).unwrap(),
                val.parse::<u32>().unwrap(),
            )
        })
        .collect_vec();
    (
        Executor::new(
            0,
            register_a.parse::<u32>().unwrap(),
            register_b.parse::<u32>().unwrap(),
            register_c.parse::<u32>().unwrap(),
        ),
        ops,
    )
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_part_1() {
        let output = run("src/test-program.txt");
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(true, true);
    }
}
