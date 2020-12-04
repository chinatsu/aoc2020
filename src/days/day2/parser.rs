use super::Password;

pub fn parse(content: &str) -> Vec<Password> {
    content.lines().filter(|val| val.len() > 1).map(|val| {
        process(val)
    }).collect::<Vec<Password>>()
}


fn process(line: &str) -> Password {
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
    use super::TEST;
    assert_eq!("abcde", parse(TEST)[0].password)
}
