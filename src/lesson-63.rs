

fn main() {
    let mut people = Hashmap::new();
    people.insert("Susan", 23);
    people.insert("Ed", 14);
    people.remove("Susan");

    match people.get("Ed") {
        Some(age) => println!("age = {:?}", age),
        None => println!("Not found")
    }

    for (person, age) in people.iter() {
        println!("person = {:?}, age = {:?}", person, age);
    }

    for person in people.keys() {
        println!("person = {:?}", person);
    }

    for age in people.values() {
        println!("age = {:?}", age);
    }
}
