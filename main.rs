use std::io::{self, Write};

fn xor_encrypt_decrypt(input: &str, key: &str) -> String {
    input
        .chars()
        .zip(key.chars().cycle())
        .map(|(i, k)| (i as u8 ^ k as u8) as char)
        .collect()
}

fn main() {
    let mut plaintext = String::new();
    let mut key = String::new();

    // Get plaintext input from the user
    print!("Enter plaintext: ");
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut plaintext).expect("Failed to read line");
    let plaintext = plaintext.trim(); 
    
    print!("Enter key: ");
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut key).expect("Failed to read line");
    let key = key.trim(); 
    
    let ciphertext = xor_encrypt_decrypt(plaintext, key);
    println!("Ciphertext: {:?}", ciphertext);

    let decrypted_text = xor_encrypt_decrypt(&ciphertext, key);
    println!("Decrypted text: {:?}", decrypted_text);
}
