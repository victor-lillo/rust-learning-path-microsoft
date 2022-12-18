use regex::Regex;

// Add the library name the [dependencies] section in Cargo.toml and then run 'cargo build'
fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
