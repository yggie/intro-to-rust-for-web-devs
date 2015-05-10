class CodeBreaker
  attr_accessor :dictionary

  def initialize(dictionary, key_length, &block)
    @dictionary = dictionary
    @cipher_factory = block
    @key_length = key_length
  end

  def crack(ciphertext, plaintext)
    counter = 0
    each_possible_dictionary_combination do |guess|
      cipher = @cipher_factory.call(guess)
      return guess if cipher.encrypt(plaintext) == ciphertext

      counter = counter + 1
      if counter % 1000 == 0
        puts %Q{Trying: "#{guess}"}
      end
    end

    raise 'Could not solve the problem!'
  end

  private

  def each_sentence_within_length(max_length)
    dictionary.each do |word|
      if word.length <= max_length
        yield(word)

        each_sentence_within_length(max_length - word.length - 1) do |small_sentence|
          yield("#{word} #{small_sentence}")
        end
      end
    end
  end

  def each_possible_dictionary_combination
    each_sentence_within_length(@key_length) do |sentence|
      yield(sentence)
    end
  end
end

require_relative './cipher/base64_encoding.rb'
require_relative './cipher/repeating_key_xor.rb'
require 'test/unit'

class TestCodeBreaker < Test::Unit::TestCase
  def test_crack
    expected_key = 'apple pie'
    cipher = Cipher::Base64Encoder.new(Cipher::RepeatingKeyXOR.new(expected_key))
    plaintext = 'This is the text to be encrypted'

    code_breaker = CodeBreaker.new(['bananas', 'apple', 'pie', 'cherry'], 10) do |key|
      Cipher::Base64Encoder.new(Cipher::RepeatingKeyXOR.new(key))
    end
    computed_key = code_breaker.crack(cipher.encrypt(plaintext), plaintext)

    assert_equal(computed_key, expected_key)
  end

  def test_crack_guesses
    expected_key = 'a long a'
    cipher = Cipher::Base64Encoder.new(Cipher::RepeatingKeyXOR.new(expected_key))
    plaintext = 'This is the text to be encrypted'
    guesses = []

    code_breaker = CodeBreaker.new(['long', 'a', 'letter'], 8) do |key|
      guesses << key
      Cipher::Base64Encoder.new(Cipher::RepeatingKeyXOR.new(key))
    end
    computed_key = code_breaker.crack(cipher.encrypt(plaintext), plaintext)

    assert_equal(expected_key, computed_key)
    assert_equal([
      'long',
      'long a',
      'long a a',
      'a',
      'a long',
      'a long a',
    ], guesses)
  end
end
