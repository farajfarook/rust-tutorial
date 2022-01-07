use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Tried to create file but there was a problem: {:?}", error),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    let _ = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    let _ = File::open("hello3.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello3.txt").expect("Tried to create file but there was a problem")
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    let username = read_username_from_file().unwrap_or_else(|error| {
        panic!("There was a problem opening the file: {:?}", error);
    });
    println!("Username is: {}", username);
    let username = read_username_from_file2().unwrap_or_else(|error| {
        panic!("There was a problem opening the file: {:?}", error);
    });
    println!("Username is: {}", username);
    let username = read_username_from_file3().unwrap_or_else(|error| {
        panic!("There was a problem opening the file: {:?}", error);
    });
    println!("Username is: {}", username);
    let username = read_username_from_file4().unwrap_or_else(|error| {
        panic!("There was a problem opening the file: {:?}", error);
    });
    println!("Username is: {}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
