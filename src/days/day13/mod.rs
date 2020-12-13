pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");

pub type Parsed = (u64, Vec<(usize, u64)>);
pub type Answer = u64;

fn solve_one(input: &Parsed) -> Answer {
    for ts in input.0..input.0+input.1[0].1 {
        for (_, bus) in &input.1 {
            if ts % bus == 0 {
                return (ts - input.0) * bus
            } 
        }
    }
    0
}

fn solve_two(input: &Parsed) -> Answer {
    let mut ts = input.0;
    while !input.1.iter().all(|(i, b)| (ts + *i as u64) % b == 0) {
        ts += input.1.iter().fold(1, |acc, (i, b)| {
            if (ts + *i as u64) % b == 0 {
                return acc * b 
            } else {
                return acc
            }
        })
    }
    ts
}

pub fn one(input: &Parsed) -> String {
    let answer = solve_one(input);
    format!("Day 13-1: {}", answer)
}

pub fn two(input: &Parsed) -> String {
    let answer = solve_two(input);
    format!("Day 13-2: {}", answer)
}

#[test]
pub fn solve_one_test() {
    assert_eq!(295, solve_one(&parser::parse(TEST)))
}


#[test]
pub fn solve_two_test() {
     assert_eq!(1068781, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(1835, solve_one(&input));
    assert_eq!(247086664214628, solve_two(&input));
}
