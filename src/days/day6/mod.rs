pub mod parser;
use std::collections::HashSet;

pub const TEST: &str = include_str!("resources/test.txt");
pub const INPUT: &str = include_str!("resources/input.txt");

fn solve_one(input: &Vec<Vec<&str>>) -> usize {
    input.iter().fold(0, |acc, element| {
        acc + element.join("").chars().collect::<HashSet<char>>().len()
    })
}

fn solve_two(input: &Vec<Vec<&str>>) -> usize {
    // big gross
    let mut sum = 0;
    input.iter().for_each(|set| {
        if set.len() == 1 {
            sum += set[0].chars().collect::<HashSet<char>>().len();
        } else {
            let answers = set.iter()
                .map(|answer| {
                    answer.chars().collect::<HashSet<char>>()
                })
                .collect::<Vec<HashSet<char>>>();
            let mut common: HashSet<char> = answers[0].clone();
            answers.iter().for_each(|answer| {
                common = common.intersection(answer).cloned().collect::<HashSet<char>>();
            });
            sum += common.len();
        }
    });
    sum
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
