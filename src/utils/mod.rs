#[macro_use] pub mod reader;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, GenericError>;

pub fn thats_weird() -> String {
    String::from("That's weird, something didn't add up")
}
