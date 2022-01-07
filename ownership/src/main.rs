fn main() {
    // Move
    let s1 = String::from("hello");
    let s2 = s1;

    // Clone
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // Copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Ownership functions
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // error: value borrowed here after move
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // Return value and ownership
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    // References and borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}", s);

    // String Slice
    let s = String::from("hello world");
    let word = first_word(&s[..]);
    println!("{}", word);
    let word = first_word(&s);
    println!("{}", word);

    let s_ltr = "hello world";
    let word = first_word(&s_ltr[..]);
    println!("{}", word);
    let word = first_word(s_ltr);
    println!("{}", word);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
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

fn change(s: &mut String) {
    s.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
