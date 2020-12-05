pub mod parser;

pub const TEST: &[u8] = include_bytes!("resources/test.txt");
pub const INPUT: &[u8] = include_bytes!("resources/input.txt");

fn solve_one(input: &Vec<BoardingPass>) -> u32 {
    input.iter().fold(0, |champion, challenger| {
        if challenger.seat_id > champion {
            challenger.seat_id
        } else {
            champion
        }
    })
}

fn solve_two(input: &Vec<BoardingPass>) -> u32 {
    let ids: Vec<u32> = input.iter().map(|pass| pass.seat_id).collect();
    let mut first_seat_encountered = false;
    for r in 0..=8096 {
        if ids.contains(&r) {
            if !first_seat_encountered {
                first_seat_encountered = true;
            }
        } else {
            if first_seat_encountered && ids.contains(&(r+1)) && ids.contains(&(r-1)) { 
                return r
            }
        }
    }
    0
}

pub fn one(input: &Vec<BoardingPass>) -> String {
    let answer = solve_one(input);
    format!("Day 5-1:  {}", answer)
}

pub fn two(input: &Vec<BoardingPass>) -> String {
    let answer = solve_two(input);
    format!("Day 5-2:  {}", answer)
}


pub struct BoardingPass {
    pub row: usize,
    pub column: usize,
    pub seat_id: u32
}

#[test]
pub fn solve_one_test() {
    assert_eq!(820, solve_one(&parser::parse(TEST)))
}

#[test]
pub fn solve_two_test() {
    // there is no seat to be found with the test set
    assert_eq!(0, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(989, solve_one(&input));
    assert_eq!(548, solve_two(&input));
}
