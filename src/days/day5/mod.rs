pub mod parser;

pub const TEST: &[u8] = include_bytes!("resources/test.txt");
pub const INPUT: &[u8] = include_bytes!("resources/input.txt");

fn solve_one(input: &Vec<u32>) -> u32 {
    *input.iter().max().unwrap()
}

fn solve_two(input: &Vec<u32>) -> u32 {
    let i: u32 = input.len() as u32 + 1;
    let mut sum: u32 = 0;
    let mut low = u32::MAX;
    for pass in input {
        if low > *pass {
            low = *pass;
        }
        sum += pass;
    }
    let high = low + i - 1;
    i * (low + high) / 2 - sum
}

pub fn one(input: &Vec<u32>) -> String {
    let answer = solve_one(input);
    format!("Day 5-1:  {}", answer)
}

pub fn two(input: &Vec<u32>) -> String {
    let answer = solve_two(input);
    format!("Day 5-2:  {}", answer)
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(989, solve_one(&input));
    assert_eq!(548, solve_two(&input));
}
