use super::{Parsed};

pub fn parse(input: &str) -> Parsed {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| (line.chars().next().unwrap(), line.chars().skip(1).collect::<String>().parse().unwrap())).collect()
}

#[test]
pub fn parse_test() {
    use super::TEST;

    assert_eq!(('F', 10), parse(TEST)[0]);
}
