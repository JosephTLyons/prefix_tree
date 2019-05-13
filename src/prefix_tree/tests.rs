use super::super::prefix_tree::PrefixTree;

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
