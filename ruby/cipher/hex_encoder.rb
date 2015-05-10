module Cipher
  class HexEncoder
    attr_reader :inner

    def initialize(cipher)
      @inner = cipher
    end

    def encrypt(plaintext)
      bytes_to_hex(@inner.encrypt(plaintext.unpack('C*')))
    end

    def decrypt(ciphertext)
      @inner.decrypt(hex_to_bytes(ciphertext)).pack('C*')
    end

    private

    def bytes_to_hex(bytes)
      bytes.map{ |byte| "%02x" % byte }.join
    end

    def hex_to_bytes(hex)
      hex.scan(/.{2}/).map{ |char| char.hex }
    end
  end
end

require_relative './null_cipher.rb'
require 'test/unit'

class TestHexEncoder < Test::Unit::TestCase
  def test_encryption
    cipher = Cipher::HexEncoder.new(Cipher::NullCipher.new)

    assert_equal(cipher.encrypt('Burning'), '4275726e696e67')
  end

  def test_decryption
    cipher = Cipher::HexEncoder.new(Cipher::NullCipher.new)

    assert_equal(cipher.decrypt('4275726e696e67'), 'Burning')
  end
end
