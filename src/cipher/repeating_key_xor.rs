use cipher::Cipher;

pub struct RepeatingKeyXOR {
    key: Vec<u8>,
}

impl RepeatingKeyXOR {
    pub fn new(key: &str) -> RepeatingKeyXOR {
        RepeatingKeyXOR {
            key: key.bytes().collect()
        }
    }
}

impl Cipher for RepeatingKeyXOR {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {
        let key_length = self.key.len();

        return (0..plaintext.len())
            .map(|index| {
                plaintext[index] ^ self.key[index % key_length]
            })
            .collect();
    }


    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        self.encrypt(ciphertext)
    }
}

#[cfg(test)]
mod tests {
    use cipher::{ Cipher, Encoder, HexEncoder, RepeatingKeyXOR };

    #[test]
    fn it_can_encrypt_correctly() {
        let cipher = HexEncoder::new(RepeatingKeyXOR::new("ICE"));

        assert_eq!(
            cipher.encrypt("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }

    #[test]
    fn it_can_decrypt_correctly() {
        let cipher = HexEncoder::new(RepeatingKeyXOR::new("ICE"));

        assert_eq!(
            cipher.decrypt("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"),
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
        );
    }
}
