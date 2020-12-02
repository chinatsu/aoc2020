pub mod parser;

fn solve_one(entries: &Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let matches = pw.password.matches(pw.target).count();
        if matches >= pw.pos1 && matches <= pw.pos2 {
            count + 1
        } else {
            count
        }
    })
}

fn solve_two(entries: &Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let first = pw.password.chars().nth(pw.pos1-1).unwrap();
        let second = pw.password.chars().nth(pw.pos2-1).unwrap();
        if (first == pw.target && second == pw.target)
        || (first != pw.target && second != pw.target) {
            count
        } else {
            count + 1
        }
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
    let entries = parser::parse("input");
    assert_eq!(556, solve_one(&entries));
    assert_eq!(605, solve_two(&entries));
}
