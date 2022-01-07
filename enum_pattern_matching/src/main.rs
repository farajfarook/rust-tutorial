fn main() {
    println!("Hello, world!");

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(home);
    dbg!(loopback);
     
    let message = Message::Write(String::from("hello"));
    message.call();
    let message = Message::Move { x: 3, y: 4 };
    message.call();
    let message = Message::Quit;
    message.call();
    let message = Message::ChangeColor(0, 255, 255);
    message.call();

    // Option
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    let num = x + y.unwrap_or_else(|| 0);
    println!("{}", num);    

    // Match
    let coin = Coin::Quarter(UsState::Alabama);
    println!("{}", coin.value_in_cents());

    // Match with options
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);

    // Catch all
    let dice_roll = 9;
    match dice_roll {
        7 => println!("You win!"),
        _ => println!("You lose!"),
    }

    // If let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Max is {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("Max is {}", max);
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}