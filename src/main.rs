use std::cell::RefCell;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

// Holds a normal char and a pointer to a Level, which is simply a vector of Letters.
struct Letter {
    letter: char,
    end_of_word: bool,
    level_below: Option<Rc<RefCell<Level>>>,
}

impl Eq for Letter {}

impl Ord for Letter {
    fn cmp(&self, other: &Letter) -> Ordering {
        self.letter.cmp(&other.letter)
    }
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

// A vector of Letters
pub struct Level {
    letter_vector: Vec<Letter>,
}

impl Level {
    // Either inserts the item if it doesn't exist and returns its location or simply returns the
    // location if it does exist.
    fn binary_insert(&mut self, plain_letter: char) -> usize {
        let letter: Letter = Letter {
            letter: plain_letter,
            end_of_word: false,
            level_below: None,
        };

        // Modified from Lucas' solution: https://stackoverflow.com/a/36253479
        match self.letter_vector.binary_search(&letter) {
            Ok(position) => position,
            Err(position) => {
                self.letter_vector.insert(position, letter);
                position
            }
        }
    }
}

#[derive(Default)]
struct Rrds {
    head: Option<Rc<RefCell<Level>>>,
    iter: Option<Rc<RefCell<Level>>>,
    word_count: u32,
}

impl Rrds {
    pub fn new() -> Self {
        let first_level = Some(Rc::new(RefCell::new(Level {
            letter_vector: Vec::new(),
        })));

        Rrds {
            head: first_level.clone(),
            iter: first_level.clone(),
            word_count: 0,
        }
    }

    pub fn insert_word(&mut self, word: &str) {
        self.word_count += 1;

        let mut position: usize;
        self.iter = self.head.clone();

        for (index, item) in word.chars().enumerate() {
            match &self.iter.clone() {
                Some(y) => {
                    // Insert Letter and get its position
                    position = y.borrow_mut().binary_insert(item);

                    // Debug code
                    // println! (
                    //     "Level: {} | Char: {} | Vector len: {}",
                    //     index + 1,
                    //     item,
                    //     y.borrow().letter_vector.len()
                    // );

                    let mut should_make_new_level: bool = false;

                    // Mark the end of the word, then we are finished, no more levels are needed
                    if index == word.len() - 1 {
                        y.borrow_mut().letter_vector[position].end_of_word = true;
                    }

                    else {
                        match y.borrow().letter_vector[position].level_below {
                            Some(_) => {
                                // Debug code
                                // println! (
                                //     "There exists a level below (Char: {} | Level: {})",
                                //     item,
                                //     index + 1)
                            }

                            // Create a new Level below
                            None => should_make_new_level = true,
                        }

                        if should_make_new_level {
                            y.borrow_mut().letter_vector[position].level_below =
                                Some(Rc::new(RefCell::new(Level {
                                    letter_vector: Vec::new(),
                                })));
                        }

                        // Move down to a level
                        self.iter = y.borrow().letter_vector[position].level_below.clone();
                    }
                }

                None => println!("Iter isn't pointing to a valid level."),
            }
        }
    }

    pub fn print_words(&mut self) {
        self.print_words_recursively(&mut self.head.clone(), String::new());
    }

    fn print_words_recursively(&mut self,
                              mut iter: &mut Option<Rc<RefCell<Level>>>,
                              mut word: String,) {
        match &mut iter {
            Some(y) => {
                for x in &mut y.borrow_mut().letter_vector {
                    word.push(x.letter);

                    if x.end_of_word {
                        println!("{}", word);
                    }

                    self.print_words_recursively(&mut x.level_below, word.clone());

                    word.pop();
                }
            }

            None => return,
        }
    }

    pub fn get_word_count(&self) -> u32 {
        self.word_count
    }
}

fn main() {
    match File::open("word_files/words_alpha.txt") {
        Ok(words_file) => {
            let buff = BufReader::new(words_file);
            let mut rrds: Rrds = Rrds::new();

            for line in buff.lines() {
                rrds.insert_word(&line.unwrap());
            }

            rrds.print_words();
            println!("Words in rrds: {}", rrds.get_word_count());
        }

        Err(e) => println!("File could not be opened: {}", e),
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

    #[test]
    fn insert_same_test() {
        let mut main_level: Level = Level {
            letter_vector: Vec::new(),
        };

        main_level.binary_insert('a');
        main_level.binary_insert('a');

        assert_eq!(true, main_level.letter_vector.len() == 1);
    }

    #[test]
    fn insert_differet_test() {
        let mut main_level: Level = Level {
            letter_vector: Vec::new(),
        };

        main_level.binary_insert('a');
        main_level.binary_insert('b');

        assert_eq!(true, main_level.letter_vector.len() == 2);
    }
}
