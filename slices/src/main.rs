fn main() {
    let s = String::from("Hello World!");

    let word = first_word(&s);
    let word2 = first_word(&s[word.len() + 1..]);

    println!("The first word is {word}");
    println!("The second word is {word2}")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
