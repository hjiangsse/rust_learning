fn main() {
    /*
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("The value of s1 is: {}", s1);
    println!("The value of s2 is: {}", s2);

    let s = String::from("hello");
    take_ownership(s);

    println!("The value of s: {}", s);
     */
    let s1 = String::from("hello world");
    /* let len = calculate_length(&s1);
    println!("The length of '{}' is '{}'", s1, len);
     */
    let hello_slice = &s1[..5];
    let world_slice = &s1[5..];
    println!("The value of hello_slice: {}", hello_slice);
    println!("The value of world_slice: {}", world_slice);
    println!("The first word: {}", first_word(&s1));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
fn calculate_length(s: &String) -> usize {
    s.len()
}
*/

/*
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
*/
