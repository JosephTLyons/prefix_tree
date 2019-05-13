mod level;

use level::Level;
use std::cell::RefCell;
use std::rc::Rc;

// Holds a normal char and a pointer to a Level, which is simply a vector of Letters.
pub struct Letter {
    letter: char,
    is_end_of_word: bool,
    level_below: Option<Rc<RefCell<Level>>>,
}
