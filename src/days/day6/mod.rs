pub mod parser;

pub const TEST: &str = include_str!("resources/test.txt");
pub const INPUT: &str = include_str!("resources/input.txt");

fn solve_one(input: &Vec<Vec<&str>>) -> usize {
    input.iter().fold(0, |acc, group| {
        acc + char_set(group).len()
    })
}

fn solve_two(input: &Vec<Vec<&str>>) -> u32 {
    input.iter().map(|group| {
        char_set(group)
            .iter()
            .fold(0, |acc, c| acc + group
                .iter()
                .all(|person| person.contains(&c.to_string())) as u32)
    }).sum()
}

fn char_set(input: &Vec<&str>) -> Vec<char> {
    let mut ret: Vec<char> = input.join("").chars().collect();
    ret.sort();
    ret.dedup();
    ret
}

pub fn one(input: &Vec<Vec<&str>>) -> String {
    let answer = solve_one(&input);
    format!("Day 6-1:  {}", answer)
}

pub fn two(input: &Vec<Vec<&str>>) -> String {
    let answer = solve_two(&input);
    format!("Day 6-2:  {}", answer)    
}

#[test]
pub fn solve_one_test() {
    assert_eq!(11, solve_one(&parser::parse(TEST)))
}


#[test]
pub fn solve_two_test() {
    assert_eq!(6, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(6504, solve_one(&input));
    assert_eq!(3351, solve_two(&input));
}
