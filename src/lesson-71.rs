fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // approach 1 for adding 1 to each element:
    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .collect();

    let new_numbers = numbers
        .iter()
        .filter(|num| num <= 3);

    let find_me = numbers
        .iter()
        .find(|num| num == 3);

    let count = numbers
        .iter()
        .count();

    let count = numbers
        .iter()
        .last();

    let count = numbers
        .iter()
        .min();

    let count = numbers
        .iter()
        .max();

    let count = numbers
        .iter()
        .take(3)
        .collect();
}
