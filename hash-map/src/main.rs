use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Alicia"), String::from("Not so bad."));
    reviews.insert(String::from("Pedro"), String::from("Excelente."));
    reviews.insert(String::from("Alberto"), String::from("Bastante bien."));

    // Look for a specific review
    let name: &str = "Pedro";
    println!("\nReview for \'{}\': {:?}\n", name, reviews.get(name));

    let obsolete: &str = "Alicia";
    println!("'{}\' removed.\n", obsolete);
    reviews.remove(obsolete);

    println!("Reviews actuales: {:#?}", reviews);
}
