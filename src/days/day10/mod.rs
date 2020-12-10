pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");

pub type Parsed = Vec<u64>;
pub type Answer = u64;

fn solve_one(input: &Parsed) -> Answer {
    let sums = input.windows(2).map(|window| window[1] - window[0]).fold(
		(0, 0),
		|(ones, threes), val| match val {
			1 => (ones + 1, threes),
			3 => (ones, threes + 1),
			_ => (ones, threes),
		},
    );
    sums.0 * sums.1
}

fn solve_two(input: &Parsed) -> Answer {
    let mut paths = vec![0u64; input.len()];
    paths[0] = 1;
    for i in 0..input.len()-1 {
        let x = input[i];
        let path = paths[i];

        for off in 0..=3 {
            if let Some(&y) = input.get(i + off) {
                if y <= x + 3 {
                    paths[i + off] += path;
                }
            }
        }
    }
    *paths.last().unwrap()
}

pub fn one(input: &Parsed) -> String {
    let answer = solve_one(input);
    format!("Day 10-1: {}", answer)
}

pub fn two(input: &Parsed) -> String {
    let answer = solve_two(input);
    format!("Day 10-2: {}", answer)
}

#[test]
pub fn solve_one_test() {
    assert_eq!(35, solve_one(&parser::parse(TEST)))
}


#[test]
pub fn solve_two_test() {
     assert_eq!(8, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(1876, solve_one(&input));
    assert_eq!(14173478093824, solve_two(&input));
}
