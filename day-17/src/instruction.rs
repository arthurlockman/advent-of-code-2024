#[allow(non_camel_case_types)]
#[derive(FromPrimitive, Debug)]
pub enum Instruction {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
}

pub trait Execute {
    fn execute(&self, operand: u32, exec: Executor) -> (Executor, Option<u32>);
}

impl Execute for Instruction {
    fn execute(&self, operand: u32, exec: Executor) -> (Executor, Option<u32>) {
        let mut exec = exec;
        let mut output: Option<u32> = None;
        let handle_combo = || -> u32 {
            // Combo operands 0 through 3 represent literal values 0 through 3.
            // Combo operand 4 represents the value of register A.
            // Combo operand 5 represents the value of register B.
            // Combo operand 6 represents the value of register C.
            // Combo operand 7 is reserved and will not appear in valid programs.
            match operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => exec.register_a,
                5 => exec.register_b,
                6 => exec.register_c,
                _ => unreachable!("No other combo operands are valid!"),
            }
        };

        match self {
            Instruction::adv => {
                // Divide A / 2^(combo operand), truncate to int and store in A, increment inst pointer by 1
                exec.register_a = exec.register_a / 2u32.pow(handle_combo());
                exec.instruction_pointer += 1;
            }
            Instruction::bxl => {
                // Bitwise XOR of B and literal operand, store in B, increment inst pointer by 1
                exec.register_b = exec.register_b ^ operand;
                exec.instruction_pointer += 1;
            }
            Instruction::bst => {
                // Combo operand % 8, store in B, increment inst pointer by 1
                exec.register_b = handle_combo() % 8;
                exec.instruction_pointer += 1;
            }
            Instruction::jnz => {
                // If A == 0 nop ; if A != 0 jmp inst pointer to literal operand
                if exec.register_a != 0 {
                    exec.instruction_pointer = operand / 2;
                } else {
                    exec.instruction_pointer += 1;
                }
            }
            Instruction::bxc => {
                // Bitwise XOR of B and C, store in B (ignore operand), increment inst pointer by 1
                exec.register_b = exec.register_b ^ exec.register_c;
                exec.instruction_pointer += 1;
            }
            Instruction::out => {
                // Combo operand % 8, output to terminal comma separated, increment inst pointer by 1
                output = Some(handle_combo() % 8);
                exec.instruction_pointer += 1;
            }
            Instruction::bdv => {
                // Divide A / 2^(combo operand), truncate to int and store in B, increment inst pointer by 1
                exec.register_b = exec.register_a / 2u32.pow(handle_combo());
                exec.instruction_pointer += 1;
            }
            Instruction::cdv => {
                // Divide A / 2^(combo operand), truncate to int and store in C, increment inst pointer by 1
                exec.register_c = exec.register_a / 2u32.pow(handle_combo());
                exec.instruction_pointer += 1;
            }
        }
        (exec, output)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Executor {
    pub instruction_pointer: u32,
    pub register_a: u32,
    pub register_b: u32,
    pub register_c: u32,
}

impl Executor {
    pub(crate) fn new(
        instruction_pointer: u32,
        register_a: u32,
        register_b: u32,
        register_c: u32,
    ) -> Executor {
        Executor {
            instruction_pointer,
            register_a,
            register_b,
            register_c,
        }
    }
}
