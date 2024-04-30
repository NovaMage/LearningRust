fn main() {
    let my_string_literal = "Welcome to my world!";

    let word = first_word(my_string_literal);

    println!("The first word is {word}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    let value: Option<i32> = Some(5);

    let _target_none: Option<u32> = None;
    println!("My option is {}", value.unwrap());

    for c in "Здравствуйте".chars() {
        println!("{c}");
    }

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let chars = s.chars();

    for (i, item) in chars.enumerate() {
        if item == ' ' {
            return &s[0..i];
        }
    }

    &s
}
