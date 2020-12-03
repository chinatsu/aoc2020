use super::{Terrain, Field};
use std::io::BufRead;

pub fn parse(filename: &str) -> Field {
    let file = std::fs::File::open(format!("src/days/day3/resources/{}.txt", filename)).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|val| val.unwrap().bytes().map(|c| {
        if c == 35 { Terrain::Tree } else { Terrain::Nothing }
    }).collect::<Vec<Terrain>>()).collect::<Vec<Vec<Terrain>>>()
}

#[test]
fn parse_test() {
    assert_eq!(Terrain::Nothing, parse("test")[0][0])
}
