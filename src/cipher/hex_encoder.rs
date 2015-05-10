extern crate rustc_serialize;

use self::rustc_serialize::hex::{ FromHex, ToHex };

use cipher::{ Cipher, Encoder };

pub struct HexEncoder<C: Cipher> {
    cipher: C,
}

impl<C> HexEncoder<C> where C: Cipher {
    pub fn new(cipher: C) -> HexEncoder<C> {
        HexEncoder {
            cipher: cipher
        }
    }
}

impl<C: Cipher> Encoder for HexEncoder<C> {
    fn encrypt(&self, plaintext: &str) -> String {
        self.cipher.encrypt(plaintext.bytes().collect()).to_hex()
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        String::from_utf8(self.cipher.decrypt(ciphertext.from_hex().unwrap())).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use cipher::{ Encoder, HexEncoder, NullCipher };

    #[test]
    fn it_can_encode_hex_correctly() {
        let encoder = HexEncoder::new(NullCipher::new());

        assert_eq!(encoder.encrypt("Burning"), "4275726e696e67");
    }

    #[test]
    fn it_can_decode_hex_correctly() {
        let encoder = HexEncoder::new(NullCipher::new());

        assert_eq!(encoder.decrypt("4275726e696e67"), "Burning");
    }
}
