use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

// Holds a normal char and a pointer to a Level, which is simply a vector of Letters.
struct Letter {
    letter: char,
    is_end_of_word: bool,
    level_below: Option<Rc<RefCell<Level>>>,
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
            is_end_of_word: false,
            level_below: None,
        };

        // Modified from Lucas' solution: https://stackoverflow.com/a/36253479
        match self.letter_vector.binary_search_by_key(&plain_letter, |letter| letter.letter) {
            Ok(position) => position,
            Err(position) => {
                self.letter_vector.insert(position, letter);
                position
            }
        }
    }
}

#[derive(Default)]
struct PrefixTree {
    head: Option<Rc<RefCell<Level>>>,
    word_count: u32,
}

impl PrefixTree {
    pub fn new() -> Self {
        PrefixTree {
            head: Some(Rc::new(RefCell::new(Level {
                letter_vector: Vec::new(),
            }))),
            word_count: 0,
        }
    }

    pub fn insert_word(&mut self, word: &str) {
        let mut position: usize;
        let mut iter: Option<Rc<RefCell<Level>>> = self.head.clone();
        let position_of_last_letter: usize = word.char_indices().count() - 1;

        for (index, character) in word.chars().enumerate() {
            match iter.clone() {
                Some(y) => {
                    // Insert Letter and get its position
                    position = y.borrow_mut().binary_insert(character);

                    // Debug code
                    // println! (
                    //     "Level: {} | Char: {} | Vector len: {}",
                    //     index + 1,
                    //     character,
                    //     y.borrow().letter_vector.len()
                    // );

                    let mut should_make_new_level: bool = false;

                    // Mark the end of the word, then we are finished, no more levels are needed
                    if index == position_of_last_letter {
                        if ! y.borrow_mut().letter_vector[position].is_end_of_word {
                            y.borrow_mut().letter_vector[position].is_end_of_word = true;
                            self.word_count += 1;
                        }
                    }

                    else {
                        match y.borrow().letter_vector[position].level_below {
                            Some(_) => {
                                // Debug code
                                // println! (
                                //     "There exists a level below (Char: {} | Level: {})",
                                //     character,
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
                        iter = y.borrow().letter_vector[position].level_below.clone();
                    }
                }

                None => println!("Iter isn't pointing to a valid level."),
            }
        }
    }

    pub fn print_all_words(&mut self) {
        self.print_words_recursively(&mut self.head.clone(), String::new());
    }

    pub fn print_all_words_with_prefix(&mut self, prefix: &str) {
        let mut iter: Option<Rc<RefCell<Level>>> = self.head.clone();
        let mut position: usize;
        let position_of_last_letter: usize = prefix.char_indices().count() - 1;

        for (index, character) in prefix.chars().enumerate() {
            match iter {
                Some(y) => {
                    position = y.borrow_mut().binary_insert(character);
                    iter = y.borrow().letter_vector[position].level_below.clone();

                    if index == position_of_last_letter
                        && y.borrow_mut().letter_vector[position].is_end_of_word {
                        println!("{}", prefix);
                    }
                }

                None => {
                    println!("Iter isn't pointing to a valid level.");
                    return;
                }
            }
        }

        self.print_words_recursively(&mut iter, prefix.to_string());
    }

    fn print_words_recursively(&mut self,
                                mut iter: &mut Option<Rc<RefCell<Level>>>,
                                mut word: String,) {
        match &mut iter {
            Some(y) => {
                for x in &mut y.borrow_mut().letter_vector {
                    word.push(x.letter);

                    if x.is_end_of_word {
                        println!("{}", word);
                    }

                    self.print_words_recursively(&mut x.level_below, word.clone());

                    word.pop();
                }
            }

            None => return,
        }
    }

    pub fn contains_word(&mut self, word: &str) -> bool {
        let mut iter: Option<Rc<RefCell<Level>>> = self.head.clone();
        let mut position: usize;
        let position_of_last_letter: usize = word.char_indices().count() - 1;

        for (index, character) in word.chars().enumerate() {
            match iter {
                Some(y) => {
                    position = y.borrow_mut().binary_insert(character);

                    if y.borrow().letter_vector[position].letter != character {
                        return false;
                    }

                    if index == position_of_last_letter
                        && y.borrow_mut().letter_vector[position].is_end_of_word {
                        return true;
                    }

                    iter = y.borrow().letter_vector[position].level_below.clone();
                }

                None => {
                    println!("Iter isn't pointing to a valid level.");
                }
            }
        }

        false
    }

    pub fn get_word_count(&self) -> u32 {
        self.word_count
    }
}

fn main() {
    match File::open("word_files/bte_lyrics.txt") {
        Ok(words_file) => {
            let buff = BufReader::new(words_file);
            let mut pt: PrefixTree = PrefixTree::new();

            for line in buff.lines() {
                pt.insert_word(&line.unwrap());
            }

            pt.print_all_words();
            println!("Words in prefix tree: {}", pt.get_word_count());
        }

        Err(e) => println!("File could not be opened: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A typical test for the contains_word() function
    fn contains_word_test_1() {
        let mut pt: PrefixTree = PrefixTree::new();
        pt.insert_word("cat");
        pt.insert_word("dog");
        pt.insert_word("fish");
        pt.insert_word("mouse");

        assert_eq!(true, pt.contains_word("dog"));
    }

    #[test]
    // This test shows that even though "dog" is a prefix of "doggy", the data structure knows it
    // isn't a valid word, since it was never inserted.
    fn contains_word_test_2() {
        let mut pt: PrefixTree = PrefixTree::new();
        pt.insert_word("doggy");

        assert_eq!(false, pt.contains_word("dog"));
    }
}
