require_relative './code_breaker.rb'

dictionary = File.read('assets/common_words.en.txt').split("\n").freeze
code_breaker = CodeBreaker.new(dictionary, 16) do |key|
  Cipher::Base64Encoder.new(Cipher::RepeatingKeyXOR.new(key))
end

plaintext = 'This is a very very secret message'
ciphertext = 'IAAMU0EZA0wEAAIXFxxUHgBSGFADCQZSEQZFCBEbFkEGFQ=='

computed_key = code_breaker.crack(ciphertext, plaintext)

puts "Boom, Ruby has determined your password to be: \"#{computed_key}\""
