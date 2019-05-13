mod letter;

use letter::Letter;

// A vector of Letters
pub struct Level {
    letter_vector: Vec<Letter>,
}

impl Level {
    // Either inserts the item if it doesn't exist and returns its location or simply returns the
    // location if it does exist.
    fn binary_insert(&mut self, plain_letter: char) -> (usize, bool) {
        let letter: Letter = Letter {
            letter: plain_letter,
            is_end_of_word: false,
            level_below: None,
        };

        // Modified from Lucas' solution: https://stackoverflow.com/a/36253479
        match self
            .letter_vector
            .binary_search_by_key(&plain_letter, |letter| letter.letter)
        {
            Ok(position) => (position, false),
            Err(position) => {
                self.letter_vector.insert(position, letter);
                (position, true)
            }
        }
    }
}
