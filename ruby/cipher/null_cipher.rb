module Cipher
  class NullCipher
    def encrypt(plaintext)
      plaintext
    end

    def decrypt(ciphertext)
      ciphertext
    end
  end
end
