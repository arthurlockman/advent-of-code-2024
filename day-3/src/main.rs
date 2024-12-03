use regex::Regex;
use utils::read_file;

fn main() {
    let corrupted_memory = read_file("src/input.txt");
    let result = uncorrupt_memory(corrupted_memory.as_str());
    println!("Uncorrupted sum: {}", result);
}

fn uncorrupt_memory(memory: &str) -> i32 {
    let instruction_re = Regex::new(r"(mul|do|don't)\((\d*),*(\d*)\)").unwrap();
    let mut result = 0;
    let mut mult_enabled = true;
    for (_, [inst, num1, num2]) in instruction_re.captures_iter(memory).map(|c| c.extract()) {
        if inst == "mul" && mult_enabled {
            let r = num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
            result += r;
        } else if inst == "do" {
            mult_enabled = true;
        } else if inst == "don't" {
            mult_enabled = false;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            uncorrupt_memory(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
            161
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            uncorrupt_memory(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }
}
