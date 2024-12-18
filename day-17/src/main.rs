#[macro_use]
extern crate num_derive;

use crate::instruction::{Execute, Executor, Instruction};
use itertools::Itertools;
use num_traits::{FromPrimitive, ToPrimitive};
use regex::Regex;
use std::collections::HashSet;
use utils::{read_file, time};

mod instruction;

fn main() {
    let (output1, time1) = time(|| run("src/input.txt"));
    println!("Part 1: {} (took {} µs)", output1, time1.as_micros());
    let (output2, time2) = time(|| run2("src/input.txt"));
    println!("Part 2: {} (took {} µs)", output2, time2.as_micros());
}

fn run(filename: &str) -> String {
    let (exec, instructions) = parse_program(filename);
    _run(exec, instructions).into_iter().join(",")
}

fn run2(filename: &str) -> u64 {
    let (exec, instructions) = parse_program(filename);
    let decoded_inst = instructions
        .iter()
        .flat_map(|i| vec![i.0.to_u64().unwrap(), i.1])
        .collect_vec();
    let mut results: HashSet<u64> = HashSet::new();
    results.insert(0);
    for (idx, _) in decoded_inst.iter().enumerate().rev() {
        let mut new_results: HashSet<u64> = HashSet::new();
        for r in results {
            // If you decompile the program it becomes apparent that the last 3 bits are what
            // drives the output value, so we can programmatically check all combinations
            // of the last 3 bits and then shift over the valid answers by 3. If we do this in
            // sequence and ensure every time that our output continues to be OK, we will
            // eventually find a valid value.
            for x in 0..=7 {
                let mut e = exec.clone();
                let test_val = (r << 3) + x;
                e.register_a = test_val;
                let result = _run(e, instructions.clone());
                if result == decoded_inst[idx..decoded_inst.len()] {
                    new_results.insert(test_val);
                }
            }
        }
        results = new_results;
    }
    *results.iter().min().unwrap()
}

fn _run(mut exec: Executor, instructions: Vec<(Instruction, u64)>) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    while (exec.instruction_pointer as usize) < instructions.len() {
        let (inst, op) = &instructions[exec.instruction_pointer as usize];
        let result = inst.execute(*op, exec);
        exec = result.0;
        if let Some(out) = result.1 {
            output.push(out);
        }
    }
    output
}

fn parse_program(filename: &str) -> (Executor, Vec<(Instruction, u64)>) {
    let file_contents = read_file(filename);
    let registers_regex =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    let program_regex = Regex::new(r"(\d),(\d)").unwrap();

    let (_, [register_a, register_b, register_c]) = registers_regex
        .captures_iter(&file_contents)
        .map(|c| c.extract())
        .collect_vec()[0];
    let ops: Vec<(Instruction, u64)> = program_regex
        .captures_iter(&file_contents)
        .map(|c| c.extract())
        .map(|(_, [op, val])| {
            (
                Instruction::from_u64(op.parse::<u64>().unwrap()).unwrap(),
                val.parse::<u64>().unwrap(),
            )
        })
        .collect_vec();
    (
        Executor::new(
            0,
            register_a.parse::<u64>().unwrap(),
            register_b.parse::<u64>().unwrap(),
            register_c.parse::<u64>().unwrap(),
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
