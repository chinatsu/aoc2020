pub mod parser;

pub const TEST: &[u8] = include_bytes!("resources/test.txt");
pub const INPUT: &[u8] = include_bytes!("resources/input.txt");

fn solve_one(input: &Vec<u32>) -> u32 {
    *input.iter().max().unwrap()
}

fn solve_two(input: &Vec<u32>) -> u32 {
    (*input.iter().min().unwrap()..*input.iter().max().unwrap())
        .filter(|r| !input.contains(&r))
        .next()
        .unwrap()
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
pub fn solve_one_test() {
    assert_eq!(820, solve_one(&parser::parse(TEST)))
}

#[test]
pub fn solve_two_test() {
    // the spec says to find a seat between two occupied seats
    // but i figure if a plane is emptier like the case with the
    // test set it's fine to just get an unoccupied seat
    assert_eq!(120, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(989, solve_one(&input));
    assert_eq!(548, solve_two(&input));
}
