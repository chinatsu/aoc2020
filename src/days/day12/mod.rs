pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");

pub type Parsed = Vec<(char, isize)>;
pub type Answer = isize;

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

fn solve_one(input: &Parsed) -> Answer {
    let mut dir = 'E';
    let mut coords: (isize, isize) = (0, 0);
    let cardinals = vec!['E', 'S', 'W', 'N'];
    for (action, value) in input {
        if action == &'F' {
            coords = new_coords(coords, directions(&dir), *value);
        }
        if cardinals.contains(action) {
            coords = new_coords(coords, directions(&action), *value);
        }
        if action == &'L' {
            let delta = 4-(*value as usize/90);
            let index = cardinals.iter().position(|c| c == &dir).unwrap();
            dir = cardinals[(index + delta) % 4];
        }
        if action == &'R' {
            let delta = *value as usize/90;
            let index = cardinals.iter().position(|c| c == &dir).unwrap();
            dir = cardinals[(index + delta) % 4];
        }
    }
    coords.0.abs() + coords.1.abs()
}

fn solve_two(input: &Parsed) -> Answer {
    let mut coords = (0isize, 0isize);
    let mut waypoint = (1isize, 10isize);
    let cardinals = vec!['E', 'S', 'W', 'N'];
    for (action, value) in input {
        if action == &'F' {
            coords = new_coords(coords, waypoint, *value);
        }
        if cardinals.contains(action) {
            waypoint = new_coords(waypoint, directions(&action), *value);
        }
        if action == &'L' {
            let delta = *value as usize/90;
            for _ in 0..delta {
                waypoint = (waypoint.1, -waypoint.0);
            }
        }
        if action == &'R' {
            let delta = *value as usize/90;
            for _ in 0..delta {
                waypoint = (-waypoint.1, waypoint.0);
            }
        }
    }
    coords.0.abs() + coords.1.abs()
}

pub fn one(input: &Parsed) -> String {
    let answer = solve_one(input);
    format!("Day 12-1: {}", answer)
}

pub fn two(input: &Parsed) -> String {
    let answer = solve_two(input);
    format!("Day 12-2: {}", answer)
}

#[test]
pub fn solve_one_test() {
    assert_eq!(25, solve_one(&parser::parse(TEST)))
}


#[test]
pub fn solve_two_test() {
     assert_eq!(286, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(759, solve_one(&input));
    assert_eq!(45763, solve_two(&input));
}
