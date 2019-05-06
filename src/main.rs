mod Dictionary;

fn main() {
    let mut dict: Dictionary::Dictionary = Dictionary::Dictionary {
        head: None,
        temp: None,
    };

    dict.create();

    dict.insert_word("Dog".to_string());
}
