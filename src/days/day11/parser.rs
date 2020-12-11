use super::{Parsed, Tile};

pub fn parse(input: &str) -> Parsed {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line
                .chars()
                .map(|c| to_tile(c))
                .collect())
        .collect()
}

fn to_tile(c: char) -> Tile {
    match c {
        '#' => Tile::Occupied,
        'L' => Tile::Available,
        _ => Tile::Floor
    }
}

#[test]
pub fn parse_test() {
    use super::TEST;

    assert_eq!(Tile::Available, parse(TEST)[0][0]);
}
