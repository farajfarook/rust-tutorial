pub fn create() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}

pub fn update() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);
}

pub fn drop() {
    {
        let v = vec![1, 2, 3];
        println!("{:?}", v);
    }
    //println!("{:?}", v);
}

pub fn read() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //v.push(6); // error
    println!("{:?}", first);
}

pub fn iterate() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

pub fn multi_type() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    println!("{:?}", row.pop().unwrap_or_else(|| SpreadsheetCell::Int(0)));
}
