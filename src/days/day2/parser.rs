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
    let mut line_iter = line.chars();
    Password {
        pos1: line_iter.by_ref().take_while(|c| c != &'-').collect::<String>().parse().unwrap(),
        pos2: line_iter.by_ref().take_while(|c| c != &' ').collect::<String>().parse().unwrap(),
        target: line_iter.by_ref().next().unwrap(),
        password: line_iter.skip(2).collect::<String>().to_string()
    }
}

#[test]
fn parse_test() {
    assert_eq!("abcde", parse("test")[0].password)
}
