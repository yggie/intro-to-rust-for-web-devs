pub trait Cipher {
    fn encrypt(&self, Vec<u8>) -> Vec<u8>;
    fn decrypt(&self, Vec<u8>) -> Vec<u8>;
}

pub trait Encoder {
    fn encrypt(&self, &str) -> String;
    fn decrypt(&self, &str) -> String;
}

pub use self::hex_encoder::HexEncoder;
pub use self::null_cipher::NullCipher;

mod hex_encoder;
mod null_cipher;
