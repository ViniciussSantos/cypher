fn shift_char(c: char, n: u32) -> char {
    if c.is_ascii() && c as u32 >= 32 && c as u32 <= 127 {
        let shifted = ((c as u32 - 32 + n) % 96 + 32) as u8;
        char::from(shifted)
    } else {
        c
    }
}

fn encode_char(key_char: char, plain_char: char) -> char {
    shift_char(plain_char, key_char as u32 - 32)
}

fn decode_char(key_char: char, encoded_char: char) -> char {
    shift_char(encoded_char, 96 - key_char as u32 + 32)
}

fn encrypt_vigenere(cypher: &str, message: &str) -> String {
    message
        .chars()
        .zip(cypher.chars().cycle())
        .map(|(plain_char, key_char)| encode_char(key_char, plain_char))
        .collect()
}

fn decrypt_vignere(cypher: &str, message: &str) -> String {
    message
        .chars()
        .zip(cypher.chars().cycle())
        .map(|(encoded_char, key_char)| decode_char(key_char, encoded_char))
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
