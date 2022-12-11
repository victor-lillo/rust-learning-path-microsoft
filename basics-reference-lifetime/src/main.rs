fn functions_lifetimes() {
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);
}

// Lifetimes in functions
// We gave them the same lifetime 'duration'
fn longest_word<'duration>(x: &'duration String, y: &'duration String) -> &'duration String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetimes in types
#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn types_lifetimes() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}

fn main() {
    functions_lifetimes();
    types_lifetimes();
}
