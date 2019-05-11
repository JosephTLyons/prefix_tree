# RRDS

## Information
RRDS = Reduced Redundancy Data Structure.  This data structure simply any amount
of text strings in a way that tries to reduce redundancy.  It is best to explain
how it reduces redundancy through an example.

```rust
fn main() {
    let mut rrds: Rrds = Rrds::new();
    rrds.insert_word(String::from("bat"));
    // ...
}
```

After `insert_word()` is executed, the internal data structure can be imagined
like this:

```txt
Level 1: b
         |
Level 2: a
         |
Level 3: t
```

Each `Level` is a vector containing a `Letter`.  The `Letter` contains a boolean
to mark the end of a word (which is used when `print_words()` is called), and a
pointer to a new level.  In this specific example, the `end_of_word` boolean for
'b' and 'a' are set to `false`, while the `end_of_word` boolean for 't' is set
to `true`.

In the next example, we insert two similar words:

```rust
fn main() {
    let mut rrds: Rrds = Rrds::new();
    rrds.insert_word(String::from("bat"));
    rrds.insert_word(String::from("ball"));
    // ...
}
```

After the second `insert_word()` is executed, the internal data structure can be
imagined like this:

```txt
Level 1: b
         |
Level 2: a
         |
Level 3: l t
         |
Level 4: l
```

In the next example, we insert two similar words and an entirely different word:

```rust
fn main() {
    let mut rrds: Rrds = Rrds::new();
    rrds.insert_word(String::from("bat"));
    rrds.insert_word(String::from("ball"));
    rrds.insert_word(String::from("zebra"));
    // ...
}
```

After the second `insert_word()` is executed, the internal data structure can be
imagined like this:

```txt
Level 1: b       z
         |       |
Level 2: a       e
         |       |
Level 3: l t     b
         |       |
Level 4: l       r
                 |
Level 5:         a
```

It should be noted that in the previous example, only `Level` 1 is shared.  
Though the diagram shows the letters on the same line for `Level`s 1 through 4,
`ball` / `bat` and `zebra` each have their own individual `Level`s.

It should be quite obvious by now that this is simply a classic tree structure,
with each node being able to grow dynamically, through a `vector`.

## Note
This data structure was mainly created to help me learn a bit more about Rust.   
It serves no purpose other really.  In theory, a data structure like this
*should* use less memory, as it attempts to use previously inserted letters from
the beginning of other words, but the implementation here may not be good enough
to not consume the advantages the theoretical model offers.  This implementation
may use **more** memory and have a fairly poor runtime execution; I haven't
analyzed it enough to entirely know.
