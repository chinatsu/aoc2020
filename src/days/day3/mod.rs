pub mod parser;

fn solve_one(field: &Field) -> u64 {
    let height = field.len() - 1;
    let width = field[0].len();
    let mut count: u64 = 0;
    let mut pos: (usize, usize) = (0, 0);
    while height > pos.0 {
        pos.0 += 1;
        pos.1 = (pos.1 + 3) % width;
        count += (field[pos.0][pos.1] == Terrain::Tree) as u64;
    }
    count
}

fn solve_two(field: &Field) -> u64 {
    let height = field.len() - 1;
    let width = field[0].len();
    let mut count: u64 = solve_one(&field);
    let checks: Vec<(usize, usize)> = vec![(1, 1), (1, 5), (1, 7), (2, 1)];
    for check in checks {
        let mut inner_count = 0;
        let mut pos: (usize, usize) = (0, 0);
        while height > pos.0 {
            pos.0 += check.0;
            pos.1 = (pos.1 + check.1) % width;
            inner_count += (field[pos.0][pos.1] == Terrain::Tree) as u64;
        }
        count *= inner_count;
    }
    count
}

pub fn one(field: &Field) -> String {
    let answer = solve_one(field);
    format!("Day 3-1:  {}", answer)
}

pub fn two(field: &Field) -> String {
    let answer = solve_two(field);
    format!("Day 3-2:  {}", answer)
}

#[derive(Debug, PartialEq)]
pub enum Terrain {
    Tree,
    Nothing
}

pub type Field = Vec<Vec<Terrain>>;

#[test]
pub fn solve_one_test() {
    assert_eq!(7, solve_one(&parser::parse("test")))
}


#[test]
pub fn solve_two_test() {
    assert_eq!(336, solve_two(&parser::parse("test")))
}

#[test]
pub fn regression() {
    let entries = parser::parse("input");
    assert_eq!(164, solve_one(&entries));
    assert_eq!(5007658656, solve_two(&entries));
}
