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

#[derive(Default)]
struct PrefixTree {
    head: Option<Rc<RefCell<Level>>>,
    word_count: u32,
    letter_count: u64,
}

impl PrefixTree {
    pub fn new() -> Self {
        PrefixTree {
            head: Some(Rc::new(RefCell::new(Level {
                letter_vector: Vec::new(),
            }))),
            word_count: 0,
            letter_count: 0,
        }
    }

    pub fn insert_word(&mut self, word: &str) {
        let mut insert_result: (usize, bool);
        let mut iter: Option<Rc<RefCell<Level>>> = self.head.clone();
        let position_of_last_letter: usize = word.char_indices().count() - 1;

        for (index, character) in word.chars().enumerate() {
            match iter.clone() {
                Some(y) => {
                    // Insert Letter and get its position
                    insert_result = y.borrow_mut().binary_insert(character);

                    if insert_result.1 {
                        self.letter_count += 1;
                    }

                    let mut should_make_new_level: bool = false;

                    // Mark the end of the word, then we are finished, no more levels are needed
                    if index == position_of_last_letter {
                        if ! y.borrow().letter_vector[insert_result.0].is_end_of_word {
                            y.borrow_mut().letter_vector[insert_result.0].is_end_of_word = true;
                            self.word_count += 1;
                        }
                    }

                    else {
                        match y.borrow().letter_vector[insert_result.0].level_below {
                            Some(_) => {},

                            // Create a new Level below
                            None => should_make_new_level = true,
                        }

                        if should_make_new_level {
                            y.borrow_mut().letter_vector[insert_result.0].level_below =
                                Some(Rc::new(RefCell::new(Level {
                                    letter_vector: Vec::new(),
                                })));
                        }

                        // Move down to a level
                        iter = y.borrow().letter_vector[insert_result.0].level_below.clone();
                    }
                }

                None => println!("Iter isn't pointing to a valid level."),
            }
        }
    }

    pub fn print_all_words(&mut self) {
        self.print_words_recursively(&mut self.head.clone(), &mut String::new());
    }

    pub fn print_all_words_with_prefix(&mut self, prefix: &str) {
        let mut iter: Option<Rc<RefCell<Level>>> = self.head.clone();
        let position_of_last_letter: usize = prefix.char_indices().count() - 1;

        for (index, character) in prefix.chars().enumerate() {
            match iter {
                Some(y) => {
                    match y
                        .borrow()
                        .letter_vector
                        .binary_search_by_key(&character, |letter| letter.letter)
                    {
                        Ok(position) => {
                            iter = y.borrow().letter_vector[position].level_below.clone();

                            if index == position_of_last_letter
                                && y.borrow().letter_vector[position].is_end_of_word {
                                println!("{}", prefix);
                            }
                        }

                        Err(_) => {
                            println!("Prefix is invalid");
                            return;
                        }
                    }
                }

                None => {
                    println!("Iter isn't pointing to a valid level.");
                    return;
                }
            }
        }

        self.print_words_recursively(&mut iter, &mut prefix.to_string());
    }

    fn print_words_recursively(&mut self,
                                iter: &mut Option<Rc<RefCell<Level>>>,
                                word: &mut String,) {
        match iter {
            Some(y) => {
                for x in &mut y.borrow_mut().letter_vector {
                    word.push(x.letter);

                    if x.is_end_of_word {
                        println!("{}", word);
                    }

                    self.print_words_recursively(&mut x.level_below, word);

                    word.pop();
                }
            }

            None => return,
        }
    }

    pub fn contains_word(&mut self, word: &str) -> bool {
        let mut iter: Option<Rc<RefCell<Level>>> = self.head.clone();
        let position_of_last_letter: usize = word.char_indices().count() - 1;

        for (index, character) in word.chars().enumerate() {
            match iter {
                Some(y) => {
                    match y
                        .borrow()
                        .letter_vector
                        .binary_search_by_key(&character, |letter| letter.letter)
                    {
                        Ok(position) => {
                            if index == position_of_last_letter
                                && y.borrow().letter_vector[position].is_end_of_word {
                                return true;
                            }

                            iter = y.borrow().letter_vector[position].level_below.clone();
                        }

                        Err(_) => {
                            return false;
                        }
                    }
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

    pub fn get_letter_count(&self) -> u64 {
        self.letter_count
    }
}

fn main() {
    let mut actual_letter_count: usize = 0;

    match File::open("word_files/bte_lyrics.txt") {
        Ok(words_file) => {
            let buff = BufReader::new(words_file);
            let mut pt: PrefixTree = PrefixTree::new();

            for line in buff.lines() {
                let result = line.unwrap();
                pt.insert_word(&result);
                actual_letter_count += result.len();
            }

            pt.print_all_words();
            println!("Letters in file: {}", actual_letter_count);
            println!("Letters in tree: {}", pt.get_letter_count());
            println!("Words in tree:   {}", pt.get_word_count());
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

    #[test]
    // A continuation on from contains_word_test_2().
    fn contains_word_test_3() {
        let mut pt: PrefixTree = PrefixTree::new();
        pt.insert_word("doggy");
        pt.insert_word("dog");

        assert_eq!(true, pt.contains_word("dog"));
    }

    #[test]
    // A simle test of get_word_count()
    fn get_word_count_test_1() {
        let mut pt: PrefixTree = PrefixTree::new();
        pt.insert_word("a");

        assert_eq!(true, pt.get_word_count() == 1);
    }

    #[test]
    // A simple test that ensures duplicate words aren't counted twice
    fn get_word_count_test_2() {
        let mut pt: PrefixTree = PrefixTree::new();
        pt.insert_word("a");
        pt.insert_word("a");

        assert_eq!(true, pt.get_word_count() == 1);
    }

    #[test]
    // A more interesting test that ensures duplicate words aren't counted twice
    fn get_word_count_test_3() {
        let mut pt: PrefixTree = PrefixTree::new();
        pt.insert_word("a");
        pt.insert_word("an");
        pt.insert_word("am");
        pt.insert_word("are");
        pt.insert_word("a");

        assert_eq!(true, pt.get_word_count() == 4);
    }

    #[test]
    // A simple get_letter_count() test
    fn get_letter_count_test_1() {
        let mut pt: PrefixTree = PrefixTree::new();
        pt.insert_word("a");

        assert_eq!(true, pt.get_letter_count() == 1);
    }

    #[test]
    // A more interesting get_letter_count() test
    fn get_letter_count_test_2() {
        let mut pt: PrefixTree = PrefixTree::new();
        pt.insert_word("a");
        pt.insert_word("an");
        pt.insert_word("am");
        pt.insert_word("are");
        pt.insert_word("as");

        assert_eq!(true, pt.get_letter_count() == 6);
    }
}
