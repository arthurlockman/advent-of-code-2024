use utils::read_file;

fn main() {
    let input = read_file("src/input.txt");
    let puzzle = parse_word_search(input);
    let part_1 = count_xmas_in_word_search(&puzzle);
    println!("Part 1: {}", part_1);
    let part_2 = count_x_mas_in_word_search(&puzzle);
    println!("Part 2: {}", part_2);
}

fn count_xmas_in_word_search(word_search: &Vec<Vec<char>>) -> u32 {
    let valid_words_lut = vec![
        vec![('X', 0, 0), ('M', 0, -1), ('A', 0, -2), ('S', 0, -3)], // up
        vec![('X', 0, 0), ('M', 1, -1), ('A', 2, -2), ('S', 3, -3)], // up, right
        vec![('X', 0, 0), ('M', 1, 0), ('A', 2, 0), ('S', 3, 0)],    // right
        vec![('X', 0, 0), ('M', 1, 1), ('A', 2, 2), ('S', 3, 3)],    // down, right
        vec![('X', 0, 0), ('M', 0, 1), ('A', 0, 2), ('S', 0, 3)],    // down
        vec![('X', 0, 0), ('M', -1, 1), ('A', -2, 2), ('S', -3, 3)], // down, left
        vec![('X', 0, 0), ('M', -1, 0), ('A', -2, 0), ('S', -3, 0)], // left
        vec![('X', 0, 0), ('M', -1, -1), ('A', -2, -2), ('S', -3, -3)] // up, left
    ];
    let mut result = 0;
    for x in 0..word_search[0].len() {
        for y in 0..word_search.len() {
            if word_search[y][x] == 'X' {
                result += match_patterns(word_search, x, y, &valid_words_lut);
            }
        }
    }
    result
}

fn count_x_mas_in_word_search(word_search: &Vec<Vec<char>>) -> u32 {
    let valid_words_lut = vec![
        vec![('A', 0, 0), ('M', -1, -1), ('S', 1, 1), ('M', 1, -1), ('S', -1, 1)],
        vec![('A', 0, 0), ('S', -1, -1), ('M', 1, 1), ('M', 1, -1), ('S', -1, 1)],
        vec![('A', 0, 0), ('M', -1, -1), ('S', 1, 1), ('S', 1, -1), ('M', -1, 1)],
        vec![('A', 0, 0), ('S', -1, -1), ('M', 1, 1), ('S', 1, -1), ('M', -1, 1)]
    ];
    let mut result = 0;
    for x in 0..word_search[0].len() {
        for y in 0..word_search.len() {
            if word_search[y][x] == 'A' {
                result += match_patterns(word_search, x, y, &valid_words_lut);
            }
        }
    }
    result
}

fn match_patterns(
    word_search: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    patterns: &Vec<Vec<(char, i32, i32)>>,
) -> u32 {
    let mut result = 0;
    for pattern in patterns {
        let mut err = false;
        for (letter, x, y) in pattern {
            let new_y = *y + start_y as i32;
            let new_x = *x + start_x as i32;
            if new_y > word_search.len() as i32 - 1
                || new_y < 0
                || new_x > word_search[0].len() as i32 - 1
                || new_x < 0
            {
                err = true;
                break;
            }
            if word_search[new_y as usize][new_x as usize] != *letter {
                err = true;
                break;
            }
        }
        if !err {
            result += 1
        }
    }
    result
}

fn parse_word_search(raw: String) -> Vec<Vec<char>> {
    raw.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        let search = parse_word_search(test_data);
        assert_eq!(count_xmas_in_word_search(&search), 18);
    }

    #[test]
    fn test_part_2() {
        let test_data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        let search = parse_word_search(test_data);
        assert_eq!(count_x_mas_in_word_search(&search), 9);
    }
}
