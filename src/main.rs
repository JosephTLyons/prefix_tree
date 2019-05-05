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
    fn insert_word(&mut self, word: String) {
        let letter: Letter;

        for x in word.chars() {
            let position: usize = self.binary_insert(x);

            // Travel down path of new letter via position
        }
    }

    fn binary_insert(&mut self, plain_letter: char) -> usize {
        let letter: Letter = Letter {
            letter: plain_letter,
            word_marker: false,
            level_below: None,
            level_above: None};

        let (position, exists): (usize, bool) = self.binary_search(plain_letter);

        if ! exists {
            self.letter_vector.insert(position, letter);
        }

        position
    }

    // Returns a position and a bool.  If the boolean is true, it already exists. If the boolean is
    // false, the position returned is where the item should be inserted.
    fn binary_search(&mut self, plain_letter: char) -> (usize, bool) {
        let letter: Letter = Letter {
            letter: plain_letter,
            word_marker: false,
            level_below: None,
            level_above: None};

        let mut lower: usize = 0;
        let mut upper: usize = self.letter_vector.len() - 1;
        let mut middle: usize = 0;

        while lower <= upper {
            middle = lower + (upper - lower) / 2;

            if self.letter_vector[middle].letter == letter.letter {
                return (middle, true);
            }

            else if self.letter_vector[middle].letter < letter.letter {
                lower = middle + 1;
            }

            else {
                upper = middle - 1;
            }
        }

        (middle, false)
    }

    // fn print(&self) {
    //
    // }
}

fn main() {
    let mut main_level: Level = Level {
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

        // main_level.binary_insert('z');
        main_level.binary_insert('a');
        // main_level.binary_insert('j');

        assert_eq!(true,    main_level.letter_vector[0].letter == 'a');
                         // && main_level.letter_vector[1].letter == 'j'
                         // && main_level.letter_vector[1].letter == 'z');
    }
}
