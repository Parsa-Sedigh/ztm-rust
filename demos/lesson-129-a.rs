struct Uppercase(String);

/* We're implementing the From trait and we're converting from String into Uppercase type. So our function parameter must be of type
String, so the data parameter is of type String.
Here, Self means Uppercase.*/
impl From<String> for Uppercase {
    fn from(data: String) -> Self {
        Uppercase(data.to_uppercase())
    }
}

impl From<&str> for Uppercase {
    fn from(data: &str) -> Self {
        Uppercase(data.to_uppercase())
    }
}

fn main() {
    let upper = Uppercase::from("lowercase");
    let upper: Uppercase = "lowercase".into();
}
