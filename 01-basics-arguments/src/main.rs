const FIRST_LETTER: char = 'A';
const LAST_LETTER: char = 'Z';

fn main() {
    // Call println! with three arguments: a string, a value, a value
    // Placeholders
    println!(
        "The first letter of the English alphabet is {} and the last letter is {}.",
        FIRST_LETTER, LAST_LETTER
    );

    println!(
        "The first letter of the English alphabet is {FIRST_LETTER} and the last letter is {LAST_LETTER}."
    );
}
