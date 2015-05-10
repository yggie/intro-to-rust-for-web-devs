extern crate rustc_serialize;

use self::rustc_serialize::base64::{ STANDARD, ToBase64, FromBase64 };
use cipher::{ Cipher, Encoder };

pub struct Base64Encoder<C> where C: Cipher {
    cipher: C
}

impl<C> Base64Encoder<C> where C: Cipher {
    pub fn new(cipher: C) -> Base64Encoder<C> {
        Base64Encoder {
            cipher: cipher
        }
    }
}

impl<C> Encoder for Base64Encoder<C> where C: Cipher {
    fn encrypt(&self, plaintext: &str) -> String {
        self.cipher.encrypt(plaintext.bytes().collect()).to_base64(STANDARD)
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        String::from_utf8(self.cipher.decrypt(ciphertext.from_base64().unwrap())).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use cipher::{ Encoder, Base64Encoder, NullCipher };

    #[test]
    fn it_can_encode_baes64_correctly() {
        let encoder = Base64Encoder::new(NullCipher::new());

        assert_eq!(encoder.encrypt("This is some text"), "VGhpcyBpcyBzb21lIHRleHQ=");
    }

    #[test]
    fn it_can_decode_baes64_correctly() {
        let encoder = Base64Encoder::new(NullCipher::new());

        assert_eq!(encoder.decrypt("VGhpcyBpcyBzb21lIHRleHQ="), "This is some text");
    }
}
