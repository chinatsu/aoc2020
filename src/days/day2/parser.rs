use std::io::BufRead;
use super::Password;

pub fn parse(filename: &str) -> Vec<Password> {
    let file = std::fs::File::open(format!("src/days/day2/resources/{}.txt", filename)).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().filter(|val| val.as_ref().unwrap().len() > 1).map(|val| {
        process(val.unwrap())
    }).collect::<Vec<Password>>()
}


fn process(line: String) -> Password {
    // friendship ended with regex, now spaghetti is my best friend
    let mut splits = line.split(": ");
    let rule = splits.nth(0).unwrap().to_string();
    let password = splits.nth(0).unwrap().to_string();
    let mut positions = rule.split("-");
    let pos1: usize = positions.nth(0).unwrap().parse().unwrap();
    let pos2: usize = positions.nth(0).unwrap().split(" ").nth(0).unwrap().parse().unwrap();
    let target: char = rule.split(" ").nth(1).unwrap().chars().nth(0).unwrap();
    (pos1, pos2, target, password)
}

#[test]
fn parse_test() {
    assert_eq!(String::from("abcde"), parse("test")[0].3)
}
