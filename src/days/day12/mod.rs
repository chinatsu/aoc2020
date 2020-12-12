pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");

pub type Parsed = Vec<(char, isize)>;
pub type Answer = (isize, isize);

fn directions(dir: &char) -> (isize, isize) {
    match dir {
        'E' => (0, 1),
        'W' => (0, -1),
        'N' => (1, 0),
        _ => (-1, 0)
    }
}

fn new_coords(current: (isize, isize), delta: (isize, isize), value: isize) -> (isize, isize) {
    (current.0 + delta.0 * value, current.1 + delta.1 * value)
}

fn solve(input: &Parsed) -> Answer {
    let mut dir = 'E';
    let mut coords1 = (0isize, 0isize);
    let mut coords2 = (0isize, 0isize);
    let mut waypoint = (1isize, 10isize);
    let cardinals = vec!['E', 'S', 'W', 'N'];
    for (action, value) in input {
        if action == &'F' {
            coords1 = new_coords(coords1, directions(&dir), *value);
            coords2 = new_coords(coords2, waypoint, *value);
        }
        if cardinals.contains(action) {
            coords1 = new_coords(coords1, directions(&action), *value);
            waypoint = new_coords(waypoint, directions(&action), *value);
        }
        if action == &'L' {
            let delta2 = *value as usize/90;
            for _ in 0..delta2 {
                waypoint = (waypoint.1, -waypoint.0);
            }
            let delta1 = 4-delta2;
            let index = cardinals.iter().position(|c| c == &dir).unwrap();
            dir = cardinals[(index + delta1) % 4];
        }
        if action == &'R' {
            let delta = *value as usize/90;
            for _ in 0..delta {
                waypoint = (-waypoint.1, waypoint.0);
            }
            let index = cardinals.iter().position(|c| c == &dir).unwrap();
            dir = cardinals[(index + delta) % 4];
        }
    }
    (coords1.0.abs() + coords1.1.abs(), coords2.0.abs() + coords2.1.abs())
}

pub fn day(input: &Parsed) -> String {
    let answer = solve(input);
    format!("Day 12-1: {}\nDay 12-2: {}", answer.0, answer.1)
}

#[test]
pub fn solve_test() {
    let answer = solve(&parser::parse(TEST));
    assert_eq!(25, answer.0);
    assert_eq!(286, answer.1);
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    let answer = solve(&input);
    assert_eq!(759, answer.0);
    assert_eq!(45763, answer.1);
}
