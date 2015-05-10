use cipher::Cipher;

pub struct NullCipher;

impl NullCipher {
    pub fn new() -> NullCipher {
        NullCipher
    }
}

impl Cipher for NullCipher {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {
        plaintext
    }

    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        ciphertext
    }
}
