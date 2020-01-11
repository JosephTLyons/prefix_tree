mod prefix_tree;

use prefix_tree::PrefixTree;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut actual_letter_count: usize = 0;

    let path_to_word_file = Path::new("word_files/my_words.txt");
    let words_file = File::open(path_to_word_file).expect("File could not be opened.");

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
