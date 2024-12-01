use utils::read_lines;

fn main() {
    let (left, right): (Vec<u32>, Vec<u32>) = read_lines("src/input.txt")
        .iter()
        .map(|line| {
            let tmp: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            (tmp[0], tmp[1])
        })
        .unzip();
    let part1 = find_distance(&left, &right);
    let part2 = find_similarity(&left, &right);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_distance(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    let mut l = left_list.to_vec();
    let mut r = right_list.to_vec();
    l.sort_unstable();
    l.reverse();
    r.sort_unstable();
    r.reverse();

    let mut result = 0;
    while l.len() > 0 && r.len() > 0 {
        let left = l.pop().unwrap();
        let right = r.pop().unwrap();
        let distance = right.abs_diff(left);
        result += distance
    }
    result
}

fn find_similarity(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    left_list.iter().map(|location|
        (right_list.iter().filter(|&x| *x == *location).count() as u32) * location
    ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let left_list = vec![3, 4, 2, 1, 3, 3];
        let right_list = vec![4, 3, 5, 3, 9, 3];
        let result = find_distance(&left_list, &right_list);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_2() {
        let left_list = vec![3, 4, 2, 1, 3, 3];
        let right_list = vec![4, 3, 5, 3, 9, 3];
        let result = find_similarity(&left_list, &right_list);
        assert_eq!(result, 31);
    }
}
