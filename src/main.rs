#[macro_use] mod utils;
use utils::Result;

fn main() -> Result<()> {
    let lines = lines_from!("99", char);
    for line in lines {
        println!("{}", line);
    }
    for field in csv_from!("100", u32) {
        println!("{}", field);
    }
    Ok(())
}
