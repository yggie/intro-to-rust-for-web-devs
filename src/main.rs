mod cipher;
mod code_breaker;

use std::io::Read;
use std::fs::File;

use cipher::{ Base64Encoder, RepeatingKeyXOR };
use code_breaker::CodeBreaker;

fn main() {
    let mut dictionary_file = File::open("assets/common_words.en.txt").unwrap();
    let mut dictionary_string = String::new();
    dictionary_file.read_to_string(&mut dictionary_string).unwrap();

    let dictionary = dictionary_string.split("\n").filter_map(|word| {
        if word.len() > 0 {
            return Some(word.to_string());
        } else {
            return None;
        }
    }).collect();
    let code_breaker = CodeBreaker::new(dictionary, 16, |guess| {
        Base64Encoder::new(RepeatingKeyXOR::new(guess))
    });

    let plaintext = "This is a very very secret message";
    let ciphertext = "IAAMU0EZA0wEAAIXFxxUHgBSGFADCQZSEQZFCBEbFkEGFQ==";
    let computed_key = code_breaker.crack(ciphertext, plaintext).unwrap();
    println!("Boom, Rust has determined your password to be: \"{}\"", computed_key);
}
