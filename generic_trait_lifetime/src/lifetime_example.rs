pub fn execute() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    println!("string1 still exists: {}", string1);

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = first(string1.as_str(), string2.as_str());
        println!("The first string is {}", result);
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = first(string1.as_str(), string2.as_str());
    }
    println!("The first string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
