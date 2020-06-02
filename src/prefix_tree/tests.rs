use super::super::prefix_tree::{Case, PrefixTree};

#[test]
// A typical test for the contains_word() function
fn contains_word_test_1() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
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
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    pt.insert_word("doggy");

    assert_eq!(false, pt.contains_word("dog"));
}

#[test]
// A continuation on from contains_word_test_2().
fn contains_word_test_3() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    pt.insert_word("doggy");
    pt.insert_word("dog");

    assert_eq!(true, pt.contains_word("dog"));
}

#[test]
// A simle test of get_word_count()
fn get_word_count_test_1() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    pt.insert_word("a");

    assert_eq!(1, pt.get_word_count());
}

#[test]
// A simple test that ensures duplicate words aren't counted twice
fn get_word_count_test_2() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    pt.insert_word("a");
    pt.insert_word("a");

    assert_eq!(1, pt.get_word_count());
}

#[test]
// A more interesting test that ensures duplicate words aren't counted twice
fn get_word_count_test_3() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    pt.insert_word("a");
    pt.insert_word("an");
    pt.insert_word("am");
    pt.insert_word("are");
    pt.insert_word("a");

    assert_eq!(4, pt.get_word_count());
}

#[test]
// A simple get_letter_count() test
fn get_letter_count_test_1() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    pt.insert_word("a");

    assert_eq!(1, pt.get_letter_count());
}

#[test]
// A more interesting get_letter_count() test
fn get_letter_count_test_2() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    pt.insert_word("a");
    pt.insert_word("an");
    pt.insert_word("am");
    pt.insert_word("are");
    pt.insert_word("as");

    assert_eq!(6, pt.get_letter_count());
}

#[test]
fn test_symbol_insert() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    let text: String = String::from("$#**   ... {} /<>");

    pt.insert_word(text.as_str());

    assert_eq!(true, pt.contains_word(text.as_str()));

    let new_text: String = text + " ^$@";
    assert_eq!(false, pt.contains_word(new_text.as_str()));

    pt.insert_word((new_text).as_str());
    assert_eq!(true, pt.contains_word(new_text.as_str()));
}

#[test]
fn case_insensitive_test() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Insensitive);
    let text = "CaSe InSeNsItIvE";

    pt.insert_word(text);
    assert_eq!(true, pt.contains_word(text));
}

#[test]
fn uppercase_test() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Uppercase);
    let text = "should be uppercase";

    pt.insert_word(text);
    assert_eq!(true, pt.contains_word(text.to_uppercase().as_str()));
}

#[test]
fn lowercase_test() {
    let mut pt: PrefixTree = PrefixTree::new(Case::Lowercase);
    let text = "SHOULD BE LOWERCASE";

    pt.insert_word(text);
    assert_eq!(true, pt.contains_word(text.to_lowercase().as_str()));
}
