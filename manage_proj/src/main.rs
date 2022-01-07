use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("key"), String::from("value"));
}

fn function1() -> Option<Result> {
    None
}

fn function2() -> Option<IoResult<u8>> {
    None
}
