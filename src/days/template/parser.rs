use super::Parsed;

pub fn parse(input: &str) -> Parsed {
    input.split('\n').filter(|line| line.len() > 0).map(|line| line.to_string()).collect()
}

#[test]
pub fn parse_test() {
    use super::TEST;

    let expected: Vec<String> = Vec::new();
    assert_eq!(expected, parse(TEST));
}
