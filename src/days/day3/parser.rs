use super::{Terrain, Field};

pub fn parse(content: &[u8]) -> Field {
    content.split(|b| b < &32).filter(|slice| slice.len() > 0).map(|val| val.iter().map(|b| {
        if b == &35 { Terrain::Tree } else { Terrain::Nothing }
    }).collect::<Vec<Terrain>>()).collect::<Field>()
}

#[test]
fn parse_test() {
    use super::TEST;
    let field = parse(TEST);
    assert_eq!(Terrain::Nothing, field[0][0]);
    assert_eq!(Terrain::Tree, field[1][0]);

}
