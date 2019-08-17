# Prefix Tree

## Information
A rusty implementation of a "trie", or a [prefix tree](https://en.wikipedia.org/wiki/Trie).
This implementation isn't entirely a pure prefix tree, as it uses some
modifications.  It is best to demonstrate how this implementation stores strings
through some visual examples:

```rust
fn main() {
    let mut pt: PrefixTree = PrefixTree::new();
    pt.insert_word("bat");
    // ...
}
```

After `insert_word()` is executed, the internal data structure can be imagined
like this:

```txt
Level 0: b
         |
Level 1: a
         |
Level 2: t
```

Each `Level` is a vector containing a `Letter`.  The `Letter` contains a boolean
to mark the end of a word (which is used when `print_words()` is called), and a
pointer to a new `Level`.  In this specific example, the `end_of_word` boolean for
'b' and 'a' are set to `false`, while the `end_of_word` boolean for 't' is set
to `true`.

In the next example, we insert two similar words:

```rust
fn main() {
    let mut pt: PrefixTree = PrefixTree::new();
    pt.insert_word("bat");
    pt.insert_word("ball");
    // ...
}
```

After the second `insert_word()` is executed, the internal data structure can be
imagined like this:

```txt
Level 0: b
         |
Level 1: a
         |
Level 2: l-t
         |
Level 3: l
```

In the next example, we insert two similar words and an entirely different word:

```rust
fn main() {
    let mut pt: PrefixTree = PrefixTree::new();
    pt.insert_word("bat");
    pt.insert_word("ball");
    pt.insert_word("zebra");
    // ...
}
```

After the second `insert_word()` is executed, the internal data structure can be
imagined like this:

```txt
Level 0: b----z
         |    |
Level 1: a    e
         |    |
Level 2: l-t  b
         |    |
Level 3: l    r
              |
Level 4:      a
```

It should be noted that in the previous example, only `Level` 1 is shared.  
Though the diagram shows the letters on the same line for `Level`s 1 through 4,
`ball` / `bat` and `zebra` each have their own individual `Level`s.

## Note
I implemented this data structure to help me learn a bit more about Rust. In
theory, a trie (prefix tree) *should* use less memory than some of the other
more conventional storage structures, as it attempts to use previously inserted
letters from the beginning of other words, but this specific implementation may
not be good enough to not outweigh the advantages the theoretical model offers.  
This implementation may use **more** memory and have a fairly poor runtime
execution; I haven't analyzed it enough to entirely know.
