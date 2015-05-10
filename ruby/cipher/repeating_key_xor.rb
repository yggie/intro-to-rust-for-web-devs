module Cipher
  class RepeatingKeyXOR
    attr_reader :key

    def initialize(key)
      @key = key
    end

    def encrypt(plaintext)
      key_bytes = key.unpack('C*')

      plaintext.each_slice(key.length).map do |slice|
        slice.each_with_index.map { |byte, index| byte ^ key_bytes[index] }
      end.flatten
    end

    def decrypt(ciphertext)
      encrypt(ciphertext)
    end
  end
end

require_relative './hex_encoder.rb'
require 'test/unit'

class TestRepeatingKeyXOR < Test::Unit::TestCase
  def test_encryption
    plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
    expected_ciphertext = '0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f'

    cipher = Cipher::HexEncoder.new(Cipher::RepeatingKeyXOR.new('ICE'))
    assert_equal(cipher.encrypt(plaintext), expected_ciphertext)
  end

  def test_decryption
    expected_plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
    ciphertext = '0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f'

    cipher = Cipher::HexEncoder.new(Cipher::RepeatingKeyXOR.new('ICE'))
    assert_equal(cipher.decrypt(ciphertext), expected_plaintext)
  end
end
