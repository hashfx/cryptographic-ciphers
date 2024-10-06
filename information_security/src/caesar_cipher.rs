use std::io::{self};

fn caesar_cipher(input: &str, shift: i32) -> String {
    let mut result = String::new();
    let shift = shift % 26;

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let first_letter = if c.is_ascii_lowercase() { 'a' } else { 'A' };
            let shifted_char = ((c as u8 - first_letter as u8 + shift as u8) % 26 + first_letter as u8) as char;
            result.push(shifted_char);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    let mut shift = String::new();

    println!("Enter the text to encrypt:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();

    println!("Enter the shift value:");
    io::stdin().read_line(&mut shift).expect("Failed to read input");
    let shift: i32 = shift.trim().parse().expect("Please enter a valid number");

    let encrypted_text = caesar_cipher(&input, shift);
    println!("Encrypted text: {}", encrypted_text);
}
