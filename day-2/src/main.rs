use utils::read_lines;

fn main() {
    let reports: Vec<Vec<u32>> = read_lines("src/input.txt")
        .iter()
        .map(|report| {
            report
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let safe_reports = reports
        .iter()
        .map(|x| validate_report_safety(x, false))
        .filter(|x| *x)
        .count();
    println!("Number of safe reports: {}", safe_reports);
    let safe_reports_2 = reports
        .iter()
        .map(|x| validate_report_safety(x, true))
        .filter(|x| *x)
        .count();
    println!(
        "Number of safe reports with the Problem Dampener: {}",
        safe_reports_2
    );
}

/// Validates a report's safety. Reports are determined safe if the following
/// conditions are met:
///
/// 1. The levels (numbers in report) are either **all increasing** or **all decreasing**.
/// 2. Any two adjacent levels differ by **at least one (1)** and **at most three (3)**.
///
/// If the input `enable_dampener` is set to `true`, reports that have
/// a **single** bad level will be allowed.
fn validate_report_safety(report: &Vec<u32>, enable_dampener: bool) -> bool {
    let mut direction = 0;
    let mut problems = 0;
    for i in 1..report.len() {
        let level = report[i];
        let minus_one = report[i - 1];
        if direction == 0 {
            // set the direction of motion, either positive or negative, based
            // on the first set of levels
            if level > minus_one {
                direction = 1
            } else if level < minus_one {
                direction = -1
            }
        }
        if level == minus_one
            || level > minus_one && direction == -1
            || level < minus_one && direction == 1
            || level.abs_diff(minus_one) > 3
        {
            problems += 1;
        }
    }
    if problems > 0 && !enable_dampener {
        false
    } else if problems > 0 && enable_dampener {
        // With the problem dampener, we can tolerate one
        // bad level. To find if that will work, remove one level at a time
        // and see if the report validates after it's removed.
        for i in 0..report.len() {
            let mut test_report = report.to_vec();
            test_report.remove(i);
            if validate_report_safety(&test_report, false) {
                return true;
            }
        }
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(validate_report_safety(&vec![7, 6, 4, 2, 1], false), true);
        assert_eq!(validate_report_safety(&vec![1, 2, 7, 8, 9], false), false);
        assert_eq!(validate_report_safety(&vec![9, 7, 6, 2, 1], false), false);
        assert_eq!(validate_report_safety(&vec![1, 3, 2, 4, 5], false), false);
        assert_eq!(validate_report_safety(&vec![8, 6, 4, 4, 1], false), false);
        assert_eq!(validate_report_safety(&vec![1, 3, 6, 7, 9], false), true);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(validate_report_safety(&vec![7, 6, 4, 2, 1], true), true);
        assert_eq!(validate_report_safety(&vec![1, 2, 7, 8, 9], true), false);
        assert_eq!(validate_report_safety(&vec![9, 7, 6, 2, 1], true), false);
        assert_eq!(validate_report_safety(&vec![1, 3, 2, 4, 5], true), true);
        assert_eq!(validate_report_safety(&vec![8, 6, 4, 4, 1], true), true);
        assert_eq!(validate_report_safety(&vec![1, 3, 6, 7, 9], true), true);
    }
}
