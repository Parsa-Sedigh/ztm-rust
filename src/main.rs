use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("chair", 5);
    stock.insert("bed", 3);
    stock.insert("table", 2);
    stock.insert("couch", 0);

    let mut total_stock = 0;

    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;

        let stock_count = if qty == &0 {
            "out of stock".to_owned();
        } else {
            format!("{:?}", qty);
        };

        println!("item={:?}, stock={:?}", item, stock_count);
    }

    println!("total stock= {:?}", total_stock);
}
