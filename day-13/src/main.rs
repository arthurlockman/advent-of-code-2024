use regex::Regex;
use utils::{read_file, time};
use z3::{Config, Context, SatResult, Solver};
use z3::ast::{Ast, Int};

struct Prize {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    z1: usize,
    z2: usize,
}

impl Prize {
    fn new(x1: &str, y1: &str, x2: &str, y2: &str, z1: &str, z2: &str) -> Self {
        Prize {
            x1: x1.parse::<usize>().unwrap(),
            y1: y1.parse::<usize>().unwrap(),
            x2: x2.parse::<usize>().unwrap(),
            y2: y2.parse::<usize>().unwrap(),
            z1: z1.parse::<usize>().unwrap(),
            z2: z2.parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    let (part1, time1) = time(|| min_tokens("src/input.txt"));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| min_tokens_2("src/input.txt"));
    println!("Part 2: {} (took {} seconds)", part2, time2.as_secs_f64());
}

fn parse_input(filename: &str) -> Vec<Prize> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let file = read_file(filename);
    let mut prizes: Vec<Prize> = Vec::new();
    for (_, [x1, y1, x2, y2, z1, z2]) in re.captures_iter(&file).map(|c| c.extract()) {
        prizes.push(Prize::new(x1, y1, x2, y2, z1, z2));
    }
    prizes
}

fn solve_prize(prize: &Prize) -> Option<(usize, usize)> {
    let mut costs: Vec<(usize, usize)> = Vec::new();
    for a in 0..=100 {
        for b in 0..=100 {
            let p1 = prize.x1 * a + prize.x2 * b;
            let p2 = prize.y1 * a + prize.y2 * b;
            if p1 > prize.z1 || p2 > prize.z2 {
                continue;
            }
            if p1 == prize.z1 && p2 == prize.z2 {
                costs.push((a, b));
            }
        }
    }
    if costs.is_empty() {
        None
    } else {
        let lowest_cost = costs
            .iter()
            .min_by(|a, b| (a.0 * 3 + a.1).cmp(&(b.0 * 3 + b.1)))
            .unwrap();
        Some(*lowest_cost)
    }
}

fn solve_prize_z3(prize: &Prize) -> Option<u64> {
    let config = Config::new();
    let context = Context::new(&config);
    let solver = Solver::new(&context);

    let x1 = Int::from_u64(&context, prize.x1 as u64);
    let x2 = Int::from_u64(&context, prize.x2 as u64);
    let y1 = Int::from_u64(&context, prize.y1 as u64);
    let y2 = Int::from_u64(&context, prize.y2 as u64);
    let z1 = Int::from_u64(&context, prize.z1 as u64);
    let z2 = Int::from_u64(&context, prize.z2 as u64);
    let a = Int::new_const(&context, "a_presses");
    let b = Int::new_const(&context, "b_presses");
    let a_cost = Int::from_u64(&context, 3);
    let b_cost = Int::from_u64(&context, 1);

    solver.assert(&(x1 * &a + x2 * &b)._eq(&z1));
    solver.assert(&(y1 * &a + y2 * &b)._eq(&z2));

    let cost = Int::new_const(&context, "cost");
    solver.assert(&cost._eq(&(a * a_cost + b * b_cost)));

    match solver.check() {
        SatResult::Sat => Some(solver
            .get_model()
            .unwrap()
            .eval(&cost, true)
            .unwrap()
            .as_u64()
            .unwrap()),
        _ => None
    }
}

fn min_tokens(filename: &str) -> usize {
    parse_input(filename)
        .iter()
        .map(|p| solve_prize(p))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .map(|p| p.0 * 3 + p.1)
        .sum::<usize>()
}

fn min_tokens_2(filename: &str) -> u64 {
    let mut prizes = parse_input(filename);
    prizes.iter_mut().for_each(|p| {
        p.z1 += 10000000000000;
        p.z2 += 10000000000000;
    });
    prizes.iter().map(|p| solve_prize_z3(p))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let tokens = min_tokens("src/test-input.txt");
        let tokens2 = min_tokens_2("src/test-input.txt");
        assert_eq!(tokens, 480);
        assert_eq!(tokens2, 875318608908);
    }
}
