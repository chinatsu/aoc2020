pub fn parse(content: &str) -> Vec<i32> {
    content.lines().map(|val| val.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[test]
fn parse_test() {
    use super::TEST;
    assert_eq!(vec![1, 2, 3], parse(TEST))
}
