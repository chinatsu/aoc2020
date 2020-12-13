use super::Parsed;

pub fn parse(input: &str) -> Parsed {
    let mut split = input.split('\n');
    let start: u64 = split.next().unwrap().parse().unwrap();
    let buses: Vec<(usize, u64)> = split
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, x)| x != &"x")
        .map(|(i, x)| (i, x.trim().parse::<u64>().unwrap()))
        .collect::<Vec<(usize, u64)>>();

    (start, buses)
}

#[test]
pub fn parse_test() {
    use super::TEST;

    assert_eq!(939, parse(TEST).0);
}
