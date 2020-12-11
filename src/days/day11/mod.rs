pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");

pub type Parsed = Vec<Vec<Tile>>;
pub type Answer = u32;

fn solve_one(input: &Parsed) -> Answer {
    let mut prev = input.clone();
    let mut cur = next_one(&input);
    while cur != prev {
        prev = cur.clone();
        cur = next_one(&cur);
    }
    let mut ret = 0;
    for line in cur {
        for seat in line {
            if seat == Tile::Occupied {
                ret += 1;
            }
        }
    }
    ret
}

fn solve_two(input: &Parsed) -> Answer {
    let mut prev = input.clone();
    let mut cur = next_two(&input);
    while cur != prev {
        prev = cur.clone();
        cur = next_two(&cur);
    }
    let mut ret = 0;
    for line in cur {
        for seat in line {
            if seat == Tile::Occupied {
                ret += 1;
            }
        }
    }
    ret
}

fn next_one(input: &Parsed) -> Parsed {
    let mut next = input.clone();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == Tile::Floor {
                continue
            }
            let surrounding = count_nearby(input, x, y);
            if input[y][x] == Tile::Available && surrounding == 0 {
                next[y][x] = Tile::Occupied;
            } else if input[y][x] == Tile::Occupied && surrounding >= 4 {
                next[y][x] = Tile::Available;
            }
        }
    }
    next.to_vec()
}

fn next_two(input: &Parsed) -> Parsed {
    let mut next = input.clone(); 
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == Tile::Floor {
                continue
            }
            let surrounding = count_visible(&input, x, y);
            if input[y][x] == Tile::Available && surrounding == 0 {
                next[y][x] = Tile::Occupied;
            } else if input[y][x] == Tile::Occupied && surrounding >= 5 {
                next[y][x] = Tile::Available
            }
        }
    }
    next.to_vec()
}

fn count_nearby(input: &Parsed, x: usize, y: usize) -> u32 {
    let mut ret = 0;
    for dx in -1_isize..=1 {
        for dy in -1_isize..=1 {
            let abs_x = x as isize + dx;
            let abs_y = y as isize + dy;
            if 0 <= abs_y && abs_y < input.len() as isize 
            && 0 <= abs_x && abs_x < input[0].len() as isize
            && !(dy == 0 && dx == 0)
            && input[abs_y as usize][abs_x as usize] == Tile::Occupied {
                ret += 1
            }
        }
    }
    ret
}

fn count_visible(input: &Parsed, x: usize, y: usize) -> u32 {
    let mut ret = 0;
    let directions: Vec<(isize, isize)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for (dx, dy) in directions {
        let (mut abs_x, mut abs_y) = (x as isize + dx, y as isize + dy);
        while 0 <= abs_y && abs_y < input.len() as isize 
        && 0 <= abs_x && abs_x < input[0].len() as isize {
            if input[abs_y as usize][abs_x as usize] == Tile::Available {
                break
            } else if input[abs_y as usize][abs_x as usize] == Tile::Occupied {
                ret += 1;
                break;
            }
            abs_x = abs_x + dx;
            abs_y = abs_y + dy;
        }
    }
    ret
}

pub fn one(input: &Parsed) -> String {
    let answer = solve_one(input);
    format!("Day 11-1: {}", answer)
}

pub fn two(input: &Parsed) -> String {
    let answer = solve_two(input);
    format!("Day 11-2: {}", answer)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Occupied, Available, Floor
}

#[test]
pub fn solve_one_test() {
    assert_eq!(37, solve_one(&parser::parse(TEST)))
}


#[test]
pub fn solve_two_test() {
     assert_eq!(26, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(2354, solve_one(&input));
    assert_eq!(2072, solve_two(&input));
}
