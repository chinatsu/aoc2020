pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");

fn solve_one(entries: &Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let matches = pw.password.matches(pw.target).count();
        count + (matches >= pw.pos1 && matches <= pw.pos2) as u32
    })
}

fn solve_two(entries: &Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let mut chars = pw.password.chars();
        count + ((chars.by_ref().nth(pw.pos1-1).unwrap() == pw.target) 
            ^ (chars.nth(pw.pos2-pw.pos1-1).unwrap() == pw.target)) as u32 
    })
}

pub fn one(entries: &Vec<Password>) -> String {
    let answer = solve_one(entries);
    format!("Day 2-1:  {}", answer)
}


pub fn two(entries: &Vec<Password>) -> String {
    let answer = solve_two(entries);
    format!("Day 2-2:  {}", answer)
}

pub struct Password {
    pos1: usize,
    pos2: usize,
    target: char,
    password: String
}

#[test]
fn regression() {
    let entries = parser::parse(INPUT);
    assert_eq!(556, solve_one(&entries));
    assert_eq!(605, solve_two(&entries));
}
