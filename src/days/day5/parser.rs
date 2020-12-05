pub fn parse(content: &[u8]) -> Vec<u32> {
    let mut res = content.split(|b| *b == 10).filter(|pass| pass.len() > 0).map(|pass| {
        pass.iter().fold(0, |n, b| match b {
            66 | 82 => 2 * n + 1,
            70 | 76 => 2 * n,
            _ => n
        })
    }).collect::<Vec<u32>>();
    res.sort_unstable();
    res
}

#[test]
pub fn parse_test() {
    use super::TEST;
    let passes = parse(TEST);
    assert_eq!(4, passes.len());

    let expected = [119, 357, 567, 820].to_vec();
    assert_eq!(expected, passes);
}
