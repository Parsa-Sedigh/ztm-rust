struct GroceryItem {
    name: String,
    qty: i32
}

fn find_grocery(name: &str) {
    let groceries = vec![
        GroceryItem {name: "22asd".to_owned(), qty: 4},
        GroceryItem {name: "sadasda".to_owned(), qty: 5},
        GroceryItem {name: "22asd".to_owned(), qty: 4},
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }

    None
}
