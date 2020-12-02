pub mod parser;

fn solve_one(entries: &Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let occurrences = pw.value.matches(pw.requirement.target).count();
        if occurrences >= pw.requirement.lower && occurrences <= pw.requirement.upper {
            count + 1
        } else {
            count
        }
    })
}

fn solve_two(entries: &Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let r = &pw.requirement;
        let first = r.lower-1;
        let second = r.upper-1;
        if (pw.value.chars().nth(first).unwrap() == r.target && pw.value.chars().nth(second).unwrap() == r.target) 
        || (pw.value.chars().nth(first).unwrap() != r.target && pw.value.chars().nth(second).unwrap() != r.target) {
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
    pub requirement: Requirement,
    pub value: String
}

pub struct Requirement {
    pub lower: usize,
    pub upper: usize,
    pub target: char
}

#[test]
fn regression() {
    let entries = parser::parse("input");
    assert_eq!(556, solve_one(&entries));
    assert_eq!(605, solve_two(&entries));
}
