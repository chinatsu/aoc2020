use std::io::BufRead;
use regex::Regex;
use super::{Password, Requirement};

pub fn parse(filename: &str) -> Vec<Password> {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w+):\s(.+)$").unwrap();
    let file = std::fs::File::open(format!("src/days/day2/resources/{}.txt", filename)).unwrap();
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
fn parse_test() {
    assert_eq!(String::from("abcde"), parse("test")[0].value)
}
