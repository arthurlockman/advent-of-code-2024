use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use utils::{read_file, time};

fn main() {
    let (part1, time1) = time(|| part_1("src/input.txt"));
    println!("Part 1: {} (took {} secs)", part1, time1.as_secs_f64());
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

struct Equation {
    left: Term,
    right: Term,
    operand: fn(Term, Term) -> u8,
    result: Term,
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
            self.result.value = Some((self.operand)(left.clone(), right.clone()));
            self.left.value = left.value;
            self.right.value = right.value;
            true
        } else {
            false
        }
    }
}

fn parse_equations(filename: &str) -> Vec<Equation> {
    let file = read_file(filename);
    let mut terms = Vec::<Term>::new();
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
                "XOR" => {
                    |left: Term, right: Term| -> u8 { left.value.unwrap() ^ right.value.unwrap() }
                }
                "AND" => {
                    |left: Term, right: Term| -> u8 { left.value.unwrap() & right.value.unwrap() }
                }
                "OR" => {
                    |left: Term, right: Term| -> u8 { left.value.unwrap() | right.value.unwrap() }
                }
                _ => unreachable!(),
            },
        })
    }
    equations
}

fn part_1(filename: &str) -> usize {
    let mut equations = parse_equations(filename);
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

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/sample.txt"), 2024);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(true, true);
    }
}
