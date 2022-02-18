struct Customer {
    age: Option<i32>,
    email: String
}

fn main() {
    let mark = Customer {
        age: Some(22),
        email: "asdasdasd".to_owned()
    };
    let becky = Customer {
        age: None,
        email: "asdad".to_owned()
    };
    match becky.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided")
    }
}
