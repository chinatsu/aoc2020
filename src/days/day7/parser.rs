use std::collections::HashMap;
use super::Tree;

pub fn parse(content: &str) -> Tree {
    let mut ret = HashMap::new();
    content
        .split('\n')
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let (left, right) = line.split_once("contain").unwrap();
            let source = format!("{} bag", left.split(" bag").next().unwrap());
            if right.contains("no other bags") {
                ret.insert(source.clone(), Vec::new());
                return
            }
            let targets: Vec<String> = right.split(", ").filter(|target| target.len() > 0).map(|target| target.trim().to_string()).collect();
            let mut destinations: Vec<(u32, String)> = Vec::new();
            for target in targets {
                let (value_str, name_raw) = target.split_once(' ').unwrap();
                let value: u32 = value_str.trim().parse().unwrap();
                let name = format!("{} bag", name_raw.split(" bag").next().unwrap());
                destinations.push((value, name));
            }
            ret.insert(source, destinations);
        });
    ret
}

#[test]
pub fn test_parser() {
    use super::TEST;

    let empty: Vec<(u32, String)> = Vec::new();
    // the last item in the file
    assert_eq!(&empty, parse(TEST).get("dotted black bag").unwrap());
}
