use super::Parsed;

pub fn parse(input: &str) -> Parsed {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

#[test]
pub fn parse_test() {
    use super::TEST;

    assert_eq!(35, parse(TEST)[0]);
}
