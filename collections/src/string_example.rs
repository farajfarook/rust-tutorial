pub fn create() {
    let data = "initial contents";
    let s = String::from(data);
    println!("{}", s);

    let s = data.to_string();
    println!("{}", s);

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);
}

pub fn update() {
    let s1 = String::from("hello");
    let mut s2 = String::from("world");

    s2.push_str("!!!!");

    let s = s1 + " " + &s2;

    println!("{}", s);

    let s3 = String::from("tic");

    let s = format!("{} {}", s3, s2);
    println!("{}", s);
}

pub fn slicing() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);
}

pub fn iterating() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
