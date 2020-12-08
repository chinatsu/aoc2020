pub mod parser;

const INPUT: &str = include_str!("resources/input.txt");
const TEST: &str = include_str!("resources/test.txt");

pub type Parsed = Vec<String>;
pub type Answer = u32;

fn solve_one(input: &Parsed) -> Answer {
    0
}

fn solve_two(input: &Parsed) -> Answer {
    0
}

pub fn one(input: &Parsed) -> String {
    let answer = solve_one(input);
    format!("x-1:  {}", answer)
}

pub fn two(input: &Parsed) -> String {
    let answer = solve_two(input);
    format!("x-2:  {}", answer)
}

#[test]
pub fn solve_one_test() {
    assert_eq!(0, solve_one(&parser::parse(TEST)))
}


#[test]
pub fn solve_two_test() {
     assert_eq!(0, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(0, solve_one(&input));
    assert_eq!(0, solve_two(&input));
}
