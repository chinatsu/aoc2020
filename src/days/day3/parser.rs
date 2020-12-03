use super::{Terrain, Field};
use std::io::Read;

pub fn parse(filename: &str) -> Field {
    let mut file = std::fs::File::open(format!("src/days/day3/resources/{}.txt", filename)).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer.split(|b| b < &32).filter(|slice| slice.len() > 0).map(|val| val.iter().map(|b| {
        if b == &35 { Terrain::Tree } else { Terrain::Nothing }
    }).collect::<Vec<Terrain>>()).collect::<Field>()
}

#[test]
fn parse_test() {
    let field = parse("test");
    assert_eq!(Terrain::Nothing, field[0][0]);
    assert_eq!(Terrain::Tree, field[1][0]);

}
