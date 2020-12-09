pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");

pub type Parsed = Vec<u64>;
pub type Answer = u64;

pub fn solve_one(input: &Parsed, preamble_size: usize) -> Answer {
    for (i, n) in input.iter().enumerate().skip(preamble_size) {
        let prev = &input[(i - preamble_size)..i];
        if !prev.iter()
            .filter(|x| x <= &n)
            .map(|x| n - x)
            .any(|target| prev.contains(&target)) {
                return *n
            }        
    }
    0
}

pub fn solve_two(input: &Parsed, target: u64) -> Answer {
    let mut left = 0;
    let mut right = 0;
    let mut sum = input[0];
    loop {
        if sum < target {
            right += 1;
            sum += input[right];
        } else if sum > target {
            sum -= input[left];
            left += 1;
        } else {
            let window = &input[left..right];
            return window.iter().min().unwrap() + window.iter().max().unwrap();
        }
    }
}

pub fn one(answer: u64) -> String {
    format!("Day 9-1:  {}", answer)
}

pub fn two(answer: u64) -> String {
    format!("Day 9-2:  {}", answer)
}

#[test]
pub fn solve_one_test() {
    assert_eq!(127, solve_one(&parser::parse(TEST), 5))
}


#[test]
pub fn solve_two_test() {
    let target = solve_one(&parser::parse(TEST), 5);
    assert_eq!(62, solve_two(&parser::parse(TEST), target))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    let target = solve_one(&input, 25);
    assert_eq!(776203571, target);
    assert_eq!(104800569, solve_two(&input, target));
}
