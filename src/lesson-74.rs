enum Color {
    Red,
    Blue,
    Green
}

fn main() {
    let maybe_user = Some("jerry");
    match maybe_user {
        Some(user) => println!("user={:?}", user),
        None => println!("no user")
    }

    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    } else {
        println!("no user");
    };

    let red = Color::Red;
    if let Color::Red = red {
        println!("it's red!");
    } else {
        println!("it's not red");
    }
}
