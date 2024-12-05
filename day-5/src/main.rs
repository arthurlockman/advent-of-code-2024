use utils::read_lines;

struct Rule {
    first_page: u32,
    second_page: u32,
}

impl Rule {
    fn new(raw_string: &String) -> Rule {
        let split: Vec<&str> = raw_string.split("|").collect();
        Rule {
            first_page: split[0].parse::<u32>().unwrap(),
            second_page: split[1].parse::<u32>().unwrap(),
        }
    }

    fn validate(&self, pages: &Vec<u32>) -> bool {
        let first_pos = pages.iter().position(|&r| r == self.first_page);
        let second_pos = pages.iter().position(|&r| r == self.second_page);
        (first_pos.is_none() || second_pos.is_none())
            || (first_pos.is_some()
                && second_pos.is_some()
                && first_pos.unwrap() < second_pos.unwrap())
    }

    fn correct(&self, pages: &Vec<u32>) -> Vec<u32> {
        let first_pos = pages.iter().position(|&r| r == self.first_page);
        let second_pos = pages.iter().position(|&r| r == self.second_page);
        if first_pos.is_some() && second_pos.is_some() {
            let mut tmp = pages.clone();
            tmp.remove(second_pos.unwrap());
            let fp = tmp.iter().position(|&r| r == self.first_page);
            let mut new_pages = tmp[0..fp.unwrap() + 1].to_vec();
            new_pages.push(self.second_page);
            let back_half = tmp[fp.unwrap() + 1..tmp.len()].iter();
            new_pages.extend(back_half);
            return new_pages;
        }
        pages.clone()
    }
}

fn main() {
    let rules: Vec<Rule> = read_lines("src/rules.txt").iter().map(Rule::new).collect();
    let pages: Vec<Vec<u32>> = parse_pages(read_lines("src/pages.txt"));
    let valid_pages = validate_pages(&rules, &pages);
    let center_sums = sum_center_pages(&valid_pages);
    println!(
        "There are {} valid pages, and their center sum is {}.",
        valid_pages.len(),
        center_sums
    );
    let broken_pages = find_broken_pages(&rules, &pages);
    let mut fixed_pages = fix_broken_pages(&rules, &broken_pages);
    while !(validate_pages(&rules, &fixed_pages).len() == broken_pages.len()) {
        fixed_pages = fix_broken_pages(&rules, &fixed_pages);
    }
    let fixed_sums = sum_center_pages(&fixed_pages);
    println!(
        "There are {} broken pages, and their center sum is {} after repair.",
        broken_pages.len(),
        fixed_sums
    );
}

fn parse_pages(pages: Vec<String>) -> Vec<Vec<u32>> {
    pages
        .iter()
        .map(|p| p.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
        .collect()
}

fn validate_pages(rules: &Vec<Rule>, pages: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    pages
        .iter()
        .filter(|&p| rules.iter().all(|r| r.validate(p)))
        .cloned()
        .collect()
}

fn find_broken_pages(rules: &Vec<Rule>, pages: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    pages
        .iter()
        .filter(|&p| !rules.iter().all(|r| r.validate(p)))
        .cloned()
        .collect()
}

fn fix_broken_pages(rules: &Vec<Rule>, pages: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    pages
        .iter()
        .map(|p| {
            let mut new_p = p.clone();
            for rule in rules {
                if !rule.validate(&new_p) {
                    new_p = rule.correct(&new_p);
                }
            }
            new_p
        })
        .collect()
}

fn sum_center_pages(pages: &Vec<Vec<u32>>) -> u32 {
    pages.iter().map(|p| p[p.len() / 2]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_lines;

    #[test]
    fn test_part_1() {
        let test_rules: Vec<Rule> = read_lines("src/test-rules.txt")
            .iter()
            .map(Rule::new)
            .collect();
        let test_pages: Vec<Vec<u32>> = parse_pages(read_lines("src/test-pages.txt"));
        let valid_pages = validate_pages(&test_rules, &test_pages);
        assert_eq!(sum_center_pages(&valid_pages), 143);
    }

    #[test]
    fn test_part_2() {
        let test_rules: Vec<Rule> = read_lines("src/test-rules.txt")
            .iter()
            .map(Rule::new)
            .collect();
        let test_pages: Vec<Vec<u32>> = parse_pages(read_lines("src/test-pages.txt"));
        let broken_pages = find_broken_pages(&test_rules, &test_pages);
        let fixed_pages: Vec<Vec<u32>> = fix_broken_pages(&test_rules, &broken_pages);
        // while !validate_pages(&test_rules, &fixed_pages).len() == broken_pages.len() {
        //     fixed_pages = fix_broken_pages(&test_rules, &broken_pages);
        // }
        assert_eq!(sum_center_pages(&fixed_pages), 123);
    }
}
