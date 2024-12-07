use std::io::repeat;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Equation {
    pub result: u128,
    components: Vec<u128>,
    pub valid: Option<bool>,
}

impl Equation {
    pub fn new(line: &String) -> Equation {
        let halves: Vec<&str> = line.split(":").collect();
        let expected_result = halves[0].parse::<u128>().unwrap();
        let components: Vec<&str> = halves[1].split_whitespace().collect();
        Equation {
            result: expected_result,
            components: components
                .iter()
                .map(|c| c.parse::<u128>().unwrap())
                .collect(),
            valid: None,
        }
    }

    pub fn validate(&mut self, operators: &Vec<char>) -> bool {
        let operands: Vec<Vec<&char>> = std::iter::repeat(operators.iter())
            .take(self.components.len() - 1)
            .multi_cartesian_product()
            .collect();
        for ops in operands {
            let mut result = self.components[0];
            for (i, &op) in ops.iter().enumerate() {
                match op {
                    '+' => result += self.components[i + 1],
                    '*' => result *= self.components[i + 1],
                    '|' => {
                        result = (result.to_string() + &self.components[i + 1].to_string())
                            .parse::<u128>()
                            .unwrap()
                    }
                    _ => (),
                }
                if result > self.result {
                    break;
                }
            }
            if result == self.result {
                self.valid = Some(true);
                return true;
            }
        }
        false
    }
}
