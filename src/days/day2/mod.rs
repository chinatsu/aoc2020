pub mod parser;

fn solve_one(entries: &Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let matches = pw.3.matches(pw.2).count();
        if matches >= pw.0 && matches <= pw.1 {
            count + 1
        } else {
            count
        }
    })
}

fn solve_two(entries: &Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let first = pw.3.chars().nth(pw.0-1).unwrap();
        let second = pw.3.chars().nth(pw.1-1).unwrap();
        if (first == pw.2 && second == pw.2) 
        || (first != pw.2 && second != pw.2) {
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

pub type Password = (usize, usize, char, String);

#[test]
fn regression() {
    let entries = parser::parse("input");
    assert_eq!(556, solve_one(&entries));
    assert_eq!(605, solve_two(&entries));
}
