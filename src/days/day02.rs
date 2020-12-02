use std::io::BufRead;
use regex::Regex;

pub struct Password {
    pub requirement: Requirement,
    pub value: String
}

pub struct Requirement {
    pub lower: usize,
    pub upper: usize,
    pub target: char
}

pub fn solve_day02_1(entries: &mut Vec<Password>) -> u32 {
    entries.iter().fold(0, |count, pw| {
        let occurrences = pw.value.matches(pw.requirement.target).count();
        if occurrences >= pw.requirement.lower && occurrences <= pw.requirement.upper {
            count + 1
        } else {
            count
        }
    })
}

pub fn solve_day02_2(entries: &mut Vec<Password>) -> u32 {
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

pub fn day02_1() -> String {
    let mut entries = passwords_from("02");
    let answer = solve_day02_1(&mut entries);
    format!("Day 2-1:  {}", answer)
}


pub fn day02_2() -> String {
    let mut entries = passwords_from("02");
    let answer = solve_day02_2(&mut entries);
    format!("Day 2-2:  {}", answer)
}

pub fn passwords_from(filename: &str) -> Vec<Password> {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w+):\s(.+)$").unwrap();
    let file = std::fs::File::open(format!("days/{}.txt", filename)).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().filter(|val| val.as_ref().unwrap().len() > 2).map(|val| {
        let line = val.unwrap().trim().to_string();
        let captures = re.captures(&line).unwrap();
        Password {
            requirement: Requirement {
                lower: (&captures[1].parse::<usize>().unwrap()).to_owned(),
                upper: (&captures[2].parse::<usize>().unwrap()).to_owned(),
                target: (&captures[3].parse::<char>().unwrap()).to_owned(),
            },
            value: (&captures[4].to_string()).to_owned()
        }
    }).collect::<Vec<Password>>()
}

#[test]
fn passwords_from_test() {
    assert_eq!(String::from("abcde"), passwords_from("test3")[0].value)
}

#[test]
fn regression() {
    let mut entries = passwords_from("02");
    assert_eq!(556, solve_day02_1(&mut entries));
    assert_eq!(605, solve_day02_2(&mut entries));
}
