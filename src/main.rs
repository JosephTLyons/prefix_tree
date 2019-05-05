struct Letter {
    letter: char,
    word_marker: bool,
    level_below: Option<Box<Level>>,
    level_above: Option<Box<Level>>,
}

struct Level {
    letter_vector: Vec<Letter>,
}

impl Level {
    fn insert_word(&self, word: String) {
        let letter: Letter;

        for x in word.chars() {
            let position: i32 = self.find_letter_in_level(x);

            if position == -1 {
                // Add letter
                // Travel down path of new letter
            }

            // else if position > 0 {
            //     // Travel down path at position
            // }
        }
    }

    fn find_letter_in_level(&self, plain_letter: char) -> i32 {
        for x in 0..self.letter_vector.len() {
            if self.letter_vector[x].letter == plain_letter {
                return x as i32
            }
        }

        -1
    }

    fn insert_letter_alphabetically(&mut self, plain_letter: char) {

        match v.binary_search(&new_elem) {
            Ok(pos) => {} // element already in vector @ `pos`
            Err(pos) => v.insert(pos, new_elem),}
}
        if self.letter_vector.is_empty() {
            let letter: Letter = Letter {
                letter: plain_letter,
                word_marker: false,
                level_below: None,
                level_above: None};

            self.letter_vector.push(letter);

            return;
        }

        for x in 0..self.letter_vector.len() {
            if plain_letter <= self.letter_vector[x + 1].letter
                && plain_letter >= self.letter_vector[x].letter {
                let letter: Letter = Letter {
                    letter: plain_letter,
                    word_marker: false,
                    level_below: None,
                    level_above: None};

                self.letter_vector.insert(x + 1, letter);

                return;
            }
        }
    }

    // fn print(&self) {
    //
    // }
}

fn main() {
    let main_level: Level = Level {
        letter_vector: Vec::new(),
    };

    main_level.insert_word ("Dog".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_test() {
        let mut main_level: Level = Level {
            letter_vector: Vec::new(),
        };

        main_level.insert_letter_alphabetically('z');
        main_level.insert_letter_alphabetically('a');
        main_level.insert_letter_alphabetically('j');

        assert_eq!(true,    main_level.letter_vector[0].letter == 'a'
                         && main_level.letter_vector[1].letter == 'j'
                         && main_level.letter_vector[1].letter == 'z');
    }
}
