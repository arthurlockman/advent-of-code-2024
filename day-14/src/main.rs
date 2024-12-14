use crate::robot::Robot;
use itertools::Itertools;
use regex::Regex;
use utils::{read_file, time};

mod point;
mod robot;

fn main() {
    let grid_size_x = 101;
    let grid_size_y = 103;
    let robots = parse_robots("src/input.txt", grid_size_x, grid_size_y);
    let (part1, time1) = time(|| safety_score(&robots, 100));
    println!("Part 1: {} (took {} seconds)", part1, time1.as_secs_f64());
    let (part2, time2) = time(|| find_easter_egg(&robots));
    println!("Part 2: {} (took {} seconds)", part2, time2.as_secs_f64());
}

fn parse_robots(filename: &str, grid_size_x: i32, grid_size_y: i32) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").unwrap();
    let file = read_file(filename);
    let mut robots: Vec<Robot> = Vec::new();
    for (_, [pos_x, pos_y, vel_x, vel_y]) in re.captures_iter(&file).map(|c| c.extract()) {
        robots.push(Robot::new(
            pos_x.parse().unwrap(),
            pos_y.parse().unwrap(),
            vel_x.parse().unwrap(),
            vel_y.parse().unwrap(),
            grid_size_x,
            grid_size_y,
        ));
    }
    robots
}

fn print_map(robots: &Vec<Robot>) {
    let grouped_robots = robots
        .iter()
        .into_group_map_by(|&r| (r.position().x, r.position().y));
    for y in 0..robots[0].position().grid_size_y {
        for x in 0..robots[0].position().grid_size_x {
            let bot_count = grouped_robots.get(&(x, y)).unwrap_or(&vec![]).len();
            if bot_count > 0 {
                print!("{}", bot_count);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn safety_score(robots: &Vec<Robot>, seconds: i32) -> usize {
    let mut r = robots.iter().cloned().collect_vec();
    for _ in 0..seconds {
        r.iter_mut().for_each(|robot| robot.tick());
    }
    r.iter()
        .filter(|rb| rb.quadrant().is_some())
        .into_group_map_by(|rb| rb.quadrant())
        .iter()
        .map(|rb| rb.1.iter().count())
        .product()
}

fn find_easter_egg(robots: &Vec<Robot>) -> usize {
    let mut ticks = 0;
    let mut r = robots.iter().cloned().collect_vec();
    while r
        .iter()
        .into_group_map_by(|&r| (r.position().x, r.position().y))
        .iter()
        .any(|rb| rb.1.len() > 1) {
        r.iter_mut().for_each(|robot| robot.tick());
        ticks += 1;
    }
    print_map(&r);
    ticks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let grid_size_x = 11;
        let grid_size_y = 7;
        let robots = parse_robots("src/test-input.txt", grid_size_x, grid_size_y);
        print_map(&robots);
        println!();
        let score = safety_score(&robots, 100);
        assert_eq!(score, 12);
    }
}
