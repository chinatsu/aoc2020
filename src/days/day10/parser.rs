use super::Parsed;

pub fn parse(input: &str) -> Parsed {
    let mut ret: Parsed = input.split('\n').filter(|line| line.len() > 0).map(|line| line.parse().unwrap()).collect();
    ret.push(0);
    ret.sort_unstable();
    ret.push(ret[ret.len() - 1] + 3);
    ret
}

#[test]
pub fn parse_test() {
    use super::TEST;

    assert_eq!(0, parse(TEST)[0]);
}
