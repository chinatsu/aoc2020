#[macro_use] mod utils;
use utils::Result;

fn main() -> Result<()> {
    Ok(())
}

#[test]
fn lines_from_99_should_be_1_2_3() {
    assert_eq!(vec![1, 2, 3], lines_from!("99", u32))
}

#[test]
fn csv_from_100_should_be_65535_65536() {
    assert_eq!(vec![65535, 65536], csv_from!("100", u32))
}
