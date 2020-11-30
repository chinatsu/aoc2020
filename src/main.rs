#[macro_use] mod utils;
use utils::Result;

fn main() -> Result<()> {
    Ok(())
}

#[test]
fn lines_from_test1_should_be_1_2_3() {
    assert_eq!(vec![1, 2, 3], lines_from!("test1", u32))
}

#[test]
fn csv_from_test2_should_be_65535_65536() {
    assert_eq!(vec![65535, 65536], csv_from!("test2", u32))
}
