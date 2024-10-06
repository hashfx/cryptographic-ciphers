fn encrypt(plaintext: &str) -> String {
    let mut row1 = String::new();
    let mut row2 = String::new();

    for (i, c) in plaintext.chars().enumerate() {
        if c.is_alphabetic() {
            if i % 2 == 0 {
                row1.push(c);
            } else {
                row2.push(c);
            }
        }
    }

    format!("{}{}", row1, row2)
}

fn decrypt(ciphertext: &str) -> String {
    let mid = (ciphertext.len() + 1) / 2; // middle point to split into two rails
    let row1 = &ciphertext[..mid];  // row 1
    let row2 = &ciphertext[mid..];  // row 2

    let mut plaintext = String::new();
    let mut iter1 = row1.chars(); // 1st row iterator
    let mut iter2 = row2.chars(); // 2nd row iterator

    for i in 0..ciphertext.len() {
        if i % 2 == 0 {   // For even positions, pick from the first rail (row1)
            if let Some(c) = iter1.next() {
                plaintext.push(c);
            }
        } else {          // For odd positions, pick from the second rail (row2)
            if let Some(c) = iter2.next() {
                plaintext.push(c);
            }
        }
    }

    plaintext
}

fn main() {
    let plaintext = "HELLO WORLD";
    let encrypted = encrypt(plaintext);
    println!("Encrypted: {}", encrypted);

    let decrypted = decrypt(&encrypted);
    println!("Decrypted: {}", decrypted);
}