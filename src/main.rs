use std::fs;

fn encrypt_vigenere(plain_text: &str, key: &str) -> String {
    let key: String = key.chars().filter(|&c| c.is_ascii_alphanumeric()).collect();
    let key = key.to_ascii_lowercase();

    let key_len = key.len();
    if key_len == 0 {
        return String::from(plain_text);
    }

    let mut index = 0;

    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii() && c.is_ascii_graphic() {
                let first = 32_u8;
                let shift = key.as_bytes()[index % key_len] - b'a';
                index += 1;
                let shifted = (c as u8)
                    .wrapping_sub(first)
                    .wrapping_add(shift)
                    .wrapping_rem(95);
                (first + shifted) as char
            } else {
                c
            }
        })
        .collect()
}

fn decrypt_vignere(ciphertext: &str, key: &str) -> String {
    // Remove all unicode and non-ascii characters from key
    let key: String = key.chars().filter(|&c| c.is_ascii_alphanumeric()).collect();
    let key = key.to_ascii_lowercase();

    let key_len = key.len();
    if key_len == 0 {
        return String::from(ciphertext);
    }

    let mut index = 0;

    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii() && c.is_ascii_graphic() {
                let first = 32_u8;
                let shift = key.as_bytes()[index % key_len] - 97;
                index += 1;
                let shifted = (c as u8)
                    .wrapping_sub(first)
                    .wrapping_add(95 - shift)
                    .wrapping_rem(95);
                (first + shifted) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let key = "key";

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 3 || args.len() < 3 {
        println!("Usage: program <encrypt|decrypt> \"<message>\"");
        std::process::exit(1);
    }

    let command = &args[1];
    let message = &args[2];

    match command.as_str() {
        "encrypt" => {
            let cipher_text = encrypt_vigenere(message, key);
            println!("Cipher text: {}", cipher_text);
        }
        "decrypt" => {
            let decrypted_text = decrypt_vignere(message, key);
            println!("Decrypted text: {}", decrypted_text);
        }
        _ => {
            println!("Usage: program <encrypt|decrypt> \"<message>\"");
            std::process::exit(1);
        }
    }
}
