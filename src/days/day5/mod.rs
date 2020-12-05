pub mod parser;

pub const TEST: &[u8] = include_bytes!("resources/test.txt");
pub const INPUT: &[u8] = include_bytes!("resources/input.txt");

fn solve_one(input: &Vec<u32>) -> u32 {
    input.iter().fold(0, |champion, challenger| std::cmp::max(*challenger, champion))
}

fn solve_two(input: &Vec<u32>) -> Option<u32> {
    (*input.iter().min().unwrap()..*input.iter().max().unwrap())
        .filter(|r| !input.contains(&r) && input.contains(&(r-1)) && input.contains(&(r+1)))
        .next()
}

pub fn one(input: &Vec<u32>) -> String {
    let answer = solve_one(input);
    format!("Day 5-1:  {}", answer)
}

pub fn two(input: &Vec<u32>) -> String {
    let answer = solve_two(input);
    match answer {
        Some(a) => format!("Day 5-2:  {}", a),
        None => String::from("Day 5-2:  Something's off..")
    }
}

#[test]
pub fn solve_one_test() {
    assert_eq!(820, solve_one(&parser::parse(TEST)))
}

#[test]
pub fn solve_two_test() {
    assert_eq!(None, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(989, solve_one(&input));
    assert_eq!(Some(548), solve_two(&input));
}
