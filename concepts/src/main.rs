fn main() {
    mutability();
    shadowing();
    data_types();
    numeric_operators();
    char_types();
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x = {}, y = {}", x, y);

    let delta = y - x; // this changes x to f32

    println!("delta = {}", delta);
}

fn numeric_operators() {
    let x = 2.0;
    let y: f32 = 3.0;

    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = x % y;

    println!(
        "sum = {}, difference = {}, product = {}, quotient = {}, remainder = {}",
        sum, difference, product, quotient, remainder
    );
}

fn char_types() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
}
