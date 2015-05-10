require 'base64'

module Cipher
  class Base64Encoder
    attr_reader :inner

    def initialize(cipher)
      @inner = cipher
    end

    def encrypt(plaintext)
      bytes_to_base64(@inner.encrypt(plaintext.unpack('C*')))
    end

    def decrypt(ciphertext)
      @inner.decrypt(base64_to_bytes(ciphertext)).pack('C*')
    end

    private

    def bytes_to_base64(bytes)
      Base64.strict_encode64(bytes.pack('C*'))
    end

    def base64_to_bytes(base64)
      Base64.strict_decode64(base64).unpack('C*')
    end
  end
end

require_relative './null_cipher.rb'
require 'test/unit'

class TestBase64Encoder < Test::Unit::TestCase
  def test_encryption
    cipher = Cipher::Base64Encoder.new(Cipher::NullCipher.new)

    assert_equal(cipher.encrypt('This is some text'), 'VGhpcyBpcyBzb21lIHRleHQ=')
  end

  def test_decryption
    cipher = Cipher::Base64Encoder.new(Cipher::NullCipher.new)

    assert_equal(cipher.decrypt('VGhpcyBpcyBzb21lIHRleHQ='), 'This is some text')
  end
end
