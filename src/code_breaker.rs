use cipher::Encoder;

struct ValueIterator<'a, T>(Option<&'a T>) where T: Clone + 'a;

impl<'a, T> ValueIterator<'a, T> where T: Clone + 'a {
    fn new(value_reference: &'a T) -> ValueIterator<'a, T> {
        ValueIterator(Some(value_reference))
    }
}

impl<'a, T> Iterator for ValueIterator<'a, T> where T: Clone + 'a {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.map(|value| {
            self.0 = None;

            return value.clone();
        })
    }
}

pub struct CodeBreaker<E, F> where E: Encoder, F: Fn(&str) -> E {
    dictionary: Vec<String>,
    cipher_factory: F,
    key_length: usize,
}

impl<E, F> CodeBreaker<E, F> where E: Encoder, F: Fn(&str) -> E {
    pub fn new(dictionary: Vec<String>, key_length: usize, cipher_factory: F) -> CodeBreaker<E, F> {
        CodeBreaker {
            dictionary: dictionary,
            key_length: key_length,
            cipher_factory: cipher_factory,
        }
    }

    pub fn crack(&self, ciphertext: &str, plaintext: &str) -> Result<String, &str> {
        let mut counter = 0;

        for guess in self.all_possible_dictionary_combinations() {
            let cipher = (self.cipher_factory)(&guess);

            if cipher.encrypt(plaintext) == ciphertext {
                return Ok(guess);
            }

            counter = counter + 1;
            if counter % 1000 == 0 {
                println!("Trying: \"{}\"", guess);
            }
        }

        return Err("The key was too hard to crack...");
    }

    fn each_sentence_within_length<'b>(&'b self, max_length: usize) -> Box<Iterator<Item=String> + 'b> {
        return Box::new(
            self.dictionary.iter()
                .filter_map(move |word| -> Option<Box<Iterator<Item=String> + 'b>> {
                    if word.len() < max_length {
                        return Some(
                            Box::new(ValueIterator::new(word).chain(
                                self.each_sentence_within_length(max_length - word.len() - 1)
                                    .map(move |next_sentence| word.clone() + " " + &next_sentence)
                            ))
                        );
                    } else if word.len() == max_length {
                        return Some(Box::new(ValueIterator::new(word)));
                    }

                    return None;
                })
                .flat_map(|iterator| iterator)
        );
    }

    fn all_possible_dictionary_combinations<'b>(&'b self) -> Box<Iterator<Item=String> + 'b> {
        self.each_sentence_within_length(self.key_length)
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use cipher::{ Cipher, Encoder, Base64Encoder, RepeatingKeyXOR };
    use code_breaker::CodeBreaker;

    #[test]
    fn it_can_crack_a_key_present_in_the_dictionary() {
        let dictionary = vec!["bananas", "apple", "pie", "cherry"].iter()
            .map(|word| word.to_string())
            .collect();
        let breaker = CodeBreaker::new(dictionary, 10, |guess| {
            Base64Encoder::new(RepeatingKeyXOR::new(guess))
        });

        let key = "apple pie";
        let plaintext = "This is the text to be encrypted";
        let cipher = Base64Encoder::new(RepeatingKeyXOR::new(key));

        let computed_key = breaker.crack(&cipher.encrypt(plaintext), plaintext).unwrap();
        assert_eq!(computed_key, key);
    }

    #[test]
    fn it_iterates_over_the_elements_as_expected() {
        let dictionary = vec!["long", "a", "letter"].iter()
            .map(|word| word.to_string())
            .collect();
        let guesses_ref = RefCell::new(Vec::new());

        {
            let breaker = CodeBreaker::new(dictionary, 8, |guess| {
                guesses_ref.borrow_mut().push(guess.to_string());
                Base64Encoder::new(RepeatingKeyXOR::new(guess))
            });

            let key = "a long a";
            let plaintext = "This is the text to be encrypted";
            let cipher = Base64Encoder::new(RepeatingKeyXOR::new(key));

            breaker.crack(&cipher.encrypt(plaintext), plaintext).unwrap();
        }

        assert_eq!(*guesses_ref.borrow(), [
            "long",
            "long a",
            "long a a",
            "a",
            "a long",
            "a long a",
        ]);
    }
}
