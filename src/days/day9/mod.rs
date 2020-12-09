pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");

pub type Parsed = Vec<u64>;
pub type Answer = u64;

fn solve_one(input: &Parsed, window: usize) -> Answer {
    for (i, n) in input.iter().enumerate().skip(window) {
        let prev = &input[(i - window)..i];
        if !prev.iter()
            .filter(|x| x <= &n)
            .map(|x| n - x)
            .any(|target| prev.contains(&target)) {
                return *n
            }        
    }
    0
}

fn solve_two(input: &Parsed, window: usize) -> Answer {
    let target = solve_one(&input, window);
    for n in 2..usize::MAX {
        if let Some(answer) = input.windows(n).find(|set| set.iter().sum::<u64>() == target) {
            return answer.iter().min().unwrap() + answer.iter().max().unwrap();
        }
    }
    0
}

pub fn one(input: &Parsed) -> String {
    let answer = solve_one(input, 25);
    format!("Day 9-1:  {}", answer)
}

pub fn two(input: &Parsed) -> String {
    let answer = solve_two(input, 25);
    format!("Day 9-2:  {}", answer)
}

#[test]
pub fn solve_one_test() {
    assert_eq!(127, solve_one(&parser::parse(TEST), 5))
}


#[test]
pub fn solve_two_test() {
     assert_eq!(62, solve_two(&parser::parse(TEST), 5))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(776203571, solve_one(&input, 25));
    assert_eq!(104800569, solve_two(&input, 25));
}
