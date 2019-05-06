use std::cmp::Ordering;
use std::rc::Rc;

struct Letter {
    letter: char,
    word_marker: bool,
    level_below: Option<Box<Level>>,
    level_above: Option<Box<Level>>,
}

impl PartialOrd for Letter {
    fn partial_cmp(&self, other: &Letter) -> Option<Ordering> {
        self.letter.partial_cmp(&other.letter)
    }
}

impl PartialEq for Letter {
    fn eq(&self, other: &Letter) -> bool {
        self.letter == other.letter
    }
}

impl Eq for Letter {}

impl Ord for Letter {
    fn cmp(&self, other: &Letter) -> Ordering {
        self.letter.cmp(&other.letter)
    }
}

pub struct Level {
    letter_vector: Vec<Letter>,
}

impl Level {
    // Either inserts the item if it doesn't exist and returns this location or simple returns the
    // location if it does exist.
    fn binary_insert(&mut self, plain_letter: char) -> usize {
        let letter: Letter = Letter {
            letter: plain_letter,
            word_marker: false,
            level_below: None,
            level_above: None,
        };

        // Modified from Lucas' solution: https://stackoverflow.com/a/36253479
        match self.letter_vector.binary_search(&letter) {
            Ok(pos) => pos,
            Err(pos) => {
                self.letter_vector.insert(pos, letter);
                pos
            }
        }
    }
}

pub struct Dictionary {
    pub head: Option<Rc<Level>>,
    pub temp: Option<Rc<Level>>,
}

impl Dictionary {
    pub fn create(&mut self) {
        self.head = Some(Rc::new(Level {
            letter_vector: Vec::new(),
        }));
    }

    pub fn insert_word(&mut self, word: String) {
        self.temp = self.head.clone();
        let mut position: usize;

        for x in word.chars() {
            match &self.temp {
                Some(y) => {
                    //position = y.binary_insert(x);

                    // new up a level below inserted letter
                    // move down this path
                    //
                    // y = y.letter_vector[position].level_below;
                }

                None => println!("Temp isn't pointing to a Level"),
            }
        }
    }

    pub fn print(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_insert_test() {
        let mut main_level: Level = Level {
            letter_vector: Vec::new(),
        };

        main_level.binary_insert('i');
        main_level.binary_insert('x');
        main_level.binary_insert('u');

        assert_eq!(
            true,
            main_level.letter_vector[0].letter == 'i'
                && main_level.letter_vector[1].letter == 'u'
                && main_level.letter_vector[2].letter == 'x'
        );
    }

    #[test]
    fn big_insert_test() {
        let mut main_level: Level = Level {
            letter_vector: Vec::new(),
        };

        main_level.binary_insert('i');
        main_level.binary_insert('x');
        main_level.binary_insert('u');
        main_level.binary_insert('m');
        main_level.binary_insert('c');
        main_level.binary_insert('h');
        main_level.binary_insert('p');
        main_level.binary_insert('z');
        main_level.binary_insert('d');
        main_level.binary_insert('y');
        main_level.binary_insert('b');
        main_level.binary_insert('w');
        main_level.binary_insert('j');
        main_level.binary_insert('e');
        main_level.binary_insert('s');
        main_level.binary_insert('q');
        main_level.binary_insert('n');
        main_level.binary_insert('f');
        main_level.binary_insert('k');
        main_level.binary_insert('v');
        main_level.binary_insert('a');
        main_level.binary_insert('l');
        main_level.binary_insert('o');
        main_level.binary_insert('g');
        main_level.binary_insert('t');

        let mut in_order: bool = true;

        for x in 1..main_level.letter_vector.len() {
            in_order = in_order
                && (main_level.letter_vector[x - 1].letter < main_level.letter_vector[x].letter);
        }

        assert_eq!(true, in_order);
    }
}

// TODO
// Finish program
// Divide code into modules
// Does each character NEED an upper level link?  If I use recursion, I won't need to travel back
// to get to the previous levels.
