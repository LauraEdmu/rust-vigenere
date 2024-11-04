//! A simple Vigenère cipher program that allows the user to encrypt or decrypt text using a provided key.
//!
//! # Functions
//!
//! - `get_cleartext`: Prompts the user to enter the text to encrypt and returns it as a lowercase, trimmed `String`.
//! - `get_ciphertext`: Prompts the user to enter the text to decrypt and returns it as a lowercase, trimmed `String`.
//! - `encrypt`: Takes a cleartext and a key, and returns the encrypted text using the Vigenère cipher algorithm.
//! - `decrypt`: Takes a ciphertext and a key, and returns the decrypted text using the Vigenère cipher algorithm.
//! - `get_key`: Prompts the user to enter the key and returns it as a lowercase, trimmed `String`.
//! - `main`: The main function that allows the user to choose between encryption and decryption, and displays the result.
//!
//! # Usage
//!
//! Run the program and follow the prompts to either encrypt or decrypt a message using the Vigenère cipher.
// A program to take input from user, and compute vigenere cipher

use std::io::{self, Write};

fn get_cleartext() -> String { // Function to get the text to encrypt
    print!("Enter the text to encrypt: ");
    io::stdout().flush().unwrap(); // Ensures prompt is displayed before input
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).unwrap();
    text = text.to_lowercase();
    text.trim().to_string() // Return the trimmed text
}

fn get_ciphertext() -> String { // Function to get the text to decrypt
    print!("Enter the text to decrypt: ");
    io::stdout().flush().unwrap(); // Ensures prompt is displayed before input
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).unwrap();
    text = text.to_lowercase();
    text.trim().to_string() // Return the trimmed text
}

fn encrypt(cleartext: &str, key: &str) -> String { // Function to encrypt the text
    let mut ciphertext = String::new();
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    for (i, byte) in cleartext.bytes().enumerate() {
        let key_byte = key_bytes[i % key_len];
        let encrypted_byte = ((byte - b'a' + key_byte - b'a') % 26) + b'a';
        ciphertext.push(encrypted_byte as char);
    }

    ciphertext
}

fn decrypt(ciphertext: &str, key: &str) -> String {
    let mut cleartext = String::new();
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    for (i, byte) in ciphertext.bytes().enumerate() {
        if byte.is_ascii_lowercase() {
            let key_byte = key_bytes[i % key_len];
            let decrypted_byte = ((26 + byte - b'a' - (key_byte - b'a')) % 26) + b'a';
            cleartext.push(decrypted_byte as char);
        } else {
            // For non-lowercase characters, add them directly to maintain the original text's structure
            cleartext.push(byte as char);
        }
    }

    cleartext
}


fn get_key() -> String {
    let mut key = String::new(); // Initialize the key variable
    print!("Enter the key: ");
    io::stdout().flush().unwrap(); // Ensures prompt is displayed before input
    io::stdin().read_line(&mut key).expect("Failed to read line"); // Read user input
    key = key.to_lowercase(); // Convert the key to lowercase
    key.trim().to_string() // Return the trimmed key
}

fn main() {    
    // Let user choose if decrypting or encrypting
    print!("Enter 1 to encrypt, 2 to decrypt: ");
    io::stdout().flush().unwrap(); // Ensures prompt is displayed before input
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    // Declare result outside of match so it's accessible later
    let result = match choice {
        "1" => { // Encrypt
            let cleartext = get_cleartext();
            let key = get_key();
            encrypt(&cleartext, &key)
        },
        "2" => { // Decrypt
            let ciphertext = get_ciphertext();
            let key = get_key();
            decrypt(&ciphertext, &key)
        },
        _ => {
            println!("Invalid choice");
            return; // Exit early if the choice is invalid
        }
    };

    // Show result
    println!("Result: {}", result);

    // Output the result to a file
    let mut file = std::fs::File::create("output.txt").unwrap();
    file.write_all(result.as_bytes()).unwrap();
}