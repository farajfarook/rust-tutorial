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