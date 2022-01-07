fn main() {
    mutability();
    shadowing();
    data_types();
    numeric_operators();
    char_types();
    compound_types();
    array_types();

    print_labeled_measurement(7, 'm');

    function_with_returns();

    if_else();
    loop_example();
    while_for_example();
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

fn compound_types() {
    let tup: (i32, f64, u8) = (500, 7.2, 1);
    println!("tup = {:?}", tup);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn array_types() {
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    print!("a = {:?}, months = {:?}", a, months);
    let b = [3; 5]; // [3, 3, 3, 3, 3]
    println!("b = {:?}", b);
    let first = a[0];
    let second = a[1];
    println!("first = {}, second = {}", first, second);

    //let index = 10;
    //et element = a[index];
    //println!("a[{}] = {}", index, element);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn function_with_returns() {
    let sum: u32 = add_numbers(5, 6);
    println!("The sum is: {}", sum);
}

fn add_numbers(a: u32, b: u32) -> u32 {
    let sum = a + b;
    sum
}

fn if_else() {
    let temp = 10;
    if temp > 30 {
        println!("It's hot outside!");
    } else if temp < 10 {
        println!("It's pretty cold outside!");
    } else {
        println!("It's ok outside!");
    }

    let value = if true { 10 } else { -1 };
    println!("value = {}", value);
}

fn loop_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    let result = loop {
        number += 1;
        if number == 10 {
            break number;
        }
    };
    println!("The result is {}", result);
}

fn while_for_example() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for elem in a {
        println!("the element value is: {}", elem);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    for number in [3; 5] {
        println!("{}!", number);
    }
}
