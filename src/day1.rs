#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| -> u32 {
            let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let first = digits.first().unwrap().to_digit(10).unwrap();
            let last = digits.last().unwrap().to_digit(10).unwrap();
            first * 10 + last
        })
        .sum()
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digit(str: &str, pos: usize) -> Option<u8> {
    let c = str.chars().nth(pos).unwrap();
    if c.is_ascii_digit() {
        return Some(c.to_digit(10).unwrap() as u8);
    }

    for (i, &num) in NUMS.iter().enumerate() {
        if str[pos..].starts_with(num) {
            return Some((i + 1) as u8);
        }
    }

    None
}

fn find_first_digit(str: &String) -> Option<u8> {
    for i in 0..str.len() {
        if let Some(digit) = get_digit(str, i) {
            return Some(digit);
        }
    }
    None
}

fn find_last_digit(str: &String) -> Option<u8> {
    for i in (0..str.len()).rev() {
        if let Some(digit) = get_digit(str, i) {
            return Some(digit);
        }
    }
    None
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| -> u32 {
            let first: u32 = find_first_digit(line).unwrap().into();
            let last: u32 = find_last_digit(line).unwrap().into();
            first * 10 + last
        })
        .sum()
}
