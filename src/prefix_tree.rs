mod letter;
mod level;

use level::Level;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PrefixTree {
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
                        if !y.borrow().letter_vector[insert_result.0].is_end_of_word {
                            y.borrow_mut().letter_vector[insert_result.0].is_end_of_word = true;
                            self.word_count += 1;
                        }
                    }
                    else {
                        match y.borrow().letter_vector[insert_result.0].level_below {
                            Some(_) => {}

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
                        iter = y.borrow().letter_vector[insert_result.0]
                            .level_below
                            .clone();
                    }
                }

                None => println!("Iter isn't pointing to a valid level."),
            }
        }
    }

    pub fn print_all_words(&mut self) {
        self.print_words_recursively(&mut self.head.clone(), &mut String::new());
    }

    #[allow(dead_code)]
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
                                && y.borrow().letter_vector[position].is_end_of_word
                            {
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

    fn print_words_recursively(
        &mut self,
        iter: &mut Option<Rc<RefCell<Level>>>,
        word: &mut String,
    ) {
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

            None => {},
        }
    }

    #[allow(dead_code)]
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
                                && y.borrow().letter_vector[position].is_end_of_word
                            {
                                return true;
                            }

                            iter = y.borrow().letter_vector[position].level_below.clone();
                        }

                        Err(_) => return false,
                    }
                }

                None => {
                    println!("Iter isn't pointing to a valid level.");
                    return false;
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

#[cfg(test)]
mod tests;
