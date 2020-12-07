use std::collections::HashMap;
pub mod parser;

pub const TEST: &str = include_str!("resources/test.txt");
pub const INPUT: &str = include_str!("resources/input.txt");

pub type Tree = HashMap<String, Vec<(u32, String)>>;

fn solve_one(input: &Tree) -> usize {
    let mut ret = recursion_one(&input, "shiny gold bag".to_string());
    ret.sort();
    ret.dedup();
    ret.len()
}

fn solve_two(input: &Tree) -> u32 {
    recursion_two(&input, "shiny gold bag".to_string()) - 1 // :thinking:
}

fn recursion_one(input: &Tree, target: String) -> Vec<String> {
    let targets: Vec<String> = input.iter().filter(|(_, bags)| {
        let colors: Vec<String> = bags.iter().map(|(_, n)| n.clone()).collect();
        return colors.contains(&target)
    }).map(|(name, _)| name.clone()).collect();
    if targets.len() == 0 { 
        return Vec::new()
    }
    let others = targets.iter().flat_map(|inner_target| recursion_one(&input, inner_target.to_string())).collect::<Vec<String>>();
    [targets, others].concat()
}

fn recursion_two(input: &Tree, target: String) -> u32 {
    let mut bags: u32 = 1;
    for item in input.get(&target).unwrap() {
        bags += item.0 * recursion_two(&input, item.1.clone())
    }
    return bags
}

pub fn one(input: &Tree) -> String {
    let answer = solve_one(&input);
    format!("Day 7-1:  {}", answer)
}

pub fn two(input: &Tree) -> String {
    let answer = solve_two(&input);
    format!("Day 7-2:  {}", answer)  
}

#[test]
pub fn solve_one_test() {
    assert_eq!(4, solve_one(&parser::parse(TEST)))
}


#[test]
pub fn solve_two_test() {
    assert_eq!(32, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(272, solve_one(&input));
    assert_eq!(172246, solve_two(&input));
}
