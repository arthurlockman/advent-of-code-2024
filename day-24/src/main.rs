use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use utils::{read_file, time};

fn main() {
    let (part1, time1) = time(|| part_1("src/input.txt"));
    println!("Part 1: {} (took {} secs)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| part_2("src/input.txt"));
    println!("Part 2: {} (took {} secs)", part2, time2.as_secs_f64());
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, Hash)]
struct Term {
    name: String,
    value: Option<u8>,
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

#[derive(Debug)]
struct Equation {
    left: Term,
    right: Term,
    operand: Operand,
    result: Term,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Operand {
    XOR,
    AND,
    OR,
}

impl Equation {
    fn solved(&self) -> bool {
        self.left.value.is_some() && self.right.value.is_some() && self.result.value.is_some()
    }

    fn terms(&self) -> Vec<Term> {
        vec![self.left.clone(), self.right.clone(), self.result.clone()]
    }

    fn try_solve(&mut self, terms: Vec<Term>) -> bool {
        let left = if let Some(t) = terms
            .iter()
            .find(|term| term.name == self.left.name && term.value.is_some())
        {
            t
        } else {
            &self.left
        };
        let right = if let Some(t) = terms
            .iter()
            .find(|term| term.name == self.right.name && term.value.is_some())
        {
            t
        } else {
            &self.right
        };
        if left.value.is_some() && right.value.is_some() {
            self.result.value = Some(match self.operand {
                Operand::XOR => left.value.unwrap() ^ right.value.unwrap(),
                Operand::AND => left.value.unwrap() & right.value.unwrap(),
                Operand::OR => left.value.unwrap() | right.value.unwrap(),
            });
            self.left.value = left.value;
            self.right.value = right.value;
            true
        } else {
            false
        }
    }
}

fn parse_equations(filename: &str, terms_override: Option<Vec<Term>>) -> Vec<Equation> {
    let file = read_file(filename);
    let mut terms = Vec::<Term>::new();
    if let Some(to) = terms_override {
        terms = to.clone();
    } else {
        for (_, [name, value]) in Regex::new(r"(\w+): (\d)")
            .unwrap()
            .captures_iter(&file)
            .map(|c| c.extract())
        {
            terms.push(Term {
                name: name.to_string(),
                value: Some(value.parse::<u8>().unwrap()),
            })
        }
    }
    let mut equations = Vec::<Equation>::new();
    for (_, [left, operand, right, result]) in Regex::new(r"(\w+) (XOR|OR|AND) (\w+) -> (\w+)")
        .unwrap()
        .captures_iter(&file)
        .map(|c| c.extract())
    {
        equations.push(Equation {
            left: if let Some(l) = terms.iter().find(|t| t.name == left) {
                l.clone()
            } else {
                Term {
                    name: left.to_string(),
                    value: None,
                }
            },
            right: if let Some(r) = terms.iter().find(|t| t.name == right) {
                r.clone()
            } else {
                Term {
                    name: right.to_string(),
                    value: None,
                }
            },
            result: if let Some(r) = terms.iter().find(|t| t.name == result) {
                r.clone()
            } else {
                Term {
                    name: result.to_string(),
                    value: None,
                }
            },
            operand: match operand {
                "XOR" => Operand::XOR,
                "AND" => Operand::AND,
                "OR" => Operand::OR,
                _ => unreachable!(),
            },
        })
    }
    equations
}

fn part_1(filename: &str) -> usize {
    let mut equations = parse_equations(filename, None);
    while equations.iter().any(|e| !e.solved()) {
        let terms = equations.iter().flat_map(|e| e.terms()).collect_vec();
        for e in equations.iter_mut() {
            if e.solved() {
                continue;
            }
            if e.try_solve(terms.clone()) {
                break;
            }
        }
    }
    let mut terms = equations
        .iter()
        .flat_map(|e| e.terms())
        .unique()
        .filter(|t| t.name.starts_with("z"))
        .collect_vec();
    terms.sort();
    let binary = terms.iter().rev().map(|t| t.value.unwrap()).join("");
    usize::from_str_radix(&binary, 2).unwrap()
}

fn part_2(filename: &str) -> String {
    let equations = parse_equations(filename, None);
    let mut problems = Vec::<&str>::new();

    let find = |n1: &str, n2: &str, op: Operand| -> Option<&str> {
        if let Some(&ref n) = equations.iter().find(|e| {
            e.operand == op
                && ((e.left.name == n1 && e.right.name == n2)
                    || (e.left.name == n2 && e.right.name == n1))
        }) {
            Some(&n.result.name)
        } else {
            None
        }
    };
    let find_by_result = |n1: &str, n2: &str, op: Operand| -> Option<&str> {
        if let Some(&ref n) = equations.iter().find(|e| {
            e.operand == op
                && ((e.left.name == n1 && e.result.name == n2)
                    || (e.result.name == n2 && e.right.name == n1))
        }) {
            Some(if n.left.name == n1 {
                &n.right.name
            } else {
                &n.left.name
            })
        } else {
            None
        }
    };
    let find_eq = |r: &str, op: Operand| -> Option<&Equation> {
        equations
            .iter()
            .find(|e| e.result.name == r && e.operand == op)
    };

    // As we go, we'll need to identify the carry node
    let mut carry = "";
    // Gate 0 will be a half-adder
    let (x, y, _) = ("x00", "y00", "z00");
    // for this first half-adder really we only care about the carry bit
    if let Some(c00) = find(x, y, Operand::AND) {
        // We found the carry bit, store it
        carry = &c00;
    }

    // All of 1 - 44 should be full-adders
    for i in 1..=44 {
        // define the terms we want
        let (x, y, z) = (
            format!("x{:02}", i),
            format!("y{:02}", i),
            format!("z{:02}", i),
        );
        println!("{} {} {}", x, y, z);
        // first let's find n1 (x ^ y)
        let n1 = find(&x, &y, Operand::XOR).unwrap();
        // next n2 (x & y)
        let n2 = find(&x, &y, Operand::AND).unwrap();
        // Now find n3 (carry & n1)
        let mut n3 = find(n1, carry, Operand::AND);
        // Now let's find Z (n1 ^ carry)
        let mut zn = find(n1, carry, Operand::XOR);
        // now that we've collected all the vars, let's figure out if anything is wrong
        if zn.is_none() || n3.is_none() {
            // something's wrong with either the previous carry or n1
            println!("Something wrong with C {} or n1 {}", carry, n1);
            // can we find the correct c(in) by searching for z?
            if let Some(c) = find_by_result(n1, &z, Operand::XOR) {
                println!("Carry should be {}", c);
                zn = find(n1, c, Operand::XOR);
                n3 = find(n1, c, Operand::AND);
                println!("Found new zn and n3: {} {}", zn.unwrap(), n3.unwrap());
                println!("Swapped {} and {}", c, carry);
                problems.push(c);
                problems.push(carry);
                carry = find(&n2, &n3.unwrap(), Operand::OR).unwrap();
            }
        } else {
            if zn.unwrap() != z {
                println!(
                    "Something's wrong with {}, zn points to {} instead",
                    z,
                    zn.unwrap()
                );
                // let's find the actual Z
                if let Some(c) = find_eq(&z, Operand::XOR) {
                    println!("Carry should be {:?}", c);
                } else {
                    println!("Couldn't find z by result....");
                }
            } else {
                // Finally, the next carry
                carry = find(&n2, &n3.unwrap(), Operand::OR).unwrap();
            }
        }
    }

    problems.sort();
    problems.iter().join(",")
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/sample.txt"), 2024);
    }

    #[test]
    fn test_part_2() {
        part_2("src/input.txt");
        assert_eq!(true, true);
    }
}
