use eq::Equation;
use utils::{read_lines, time};

mod eq;

fn main() {
    let part_1 = time(|| validate_equations("src/input.txt", vec!['+', '*']));
    println!("Part 1 result: {} (took {} seconds)", part_1.0, part_1.1.as_secs_f64());
    let part_2 = time(|| validate_equations("src/input.txt", vec!['+', '*', '|']));
    println!("Part 2 result: {} (took {} seconds)", part_2.0, part_2.1.as_secs_f64());
}

fn validate_equations(filename: &str, operators: Vec<char>) -> u128 {
    let mut equations: Vec<Equation> = read_lines(filename)
            .iter()
            .map(Equation::new)
            .collect();
    for eq in &mut equations {
        eq.validate(&operators);
    }
    equations.iter().filter(|&e| e.valid.unwrap_or(false)).map(|e| e.result).sum()
}

#[cfg(test)]
mod tests {
    use eq::Equation;
    use utils::read_lines;

    use super::*;

    #[test]
    fn test_part_1() {
        let result = validate_equations("src/example.txt", vec!['+', '*']);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_2() {
        let result = validate_equations("src/example.txt", vec!['+', '*', '|']);
        assert_eq!(result, 11387);
    }
}
