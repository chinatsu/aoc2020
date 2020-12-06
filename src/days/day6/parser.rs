pub fn parse(content: &str) -> Vec<Vec<&str>> {
    content
        .split("\n\n")
        .map(|answer| answer
            .split('\n')
            .filter(|a| a != &"") // something like this is needed :(
            .collect::<Vec<&str>>())
        .collect()
}


#[test]
pub fn test_parser() {
    use super::TEST;
    let answers = parse(TEST);

    assert_eq!(5, answers.len());
}
