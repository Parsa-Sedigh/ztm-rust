fn main() {
    let mut data = Some(3);
    while let Some(i) = data {
        println!("loop");
        data = None;
    }
    println!("done");

    //while let with iterators is extremely useful:
    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter();

    // The iterable needs to be mutable since it'll be mutating.
    while let Some(num) = number_iter.next() {
        println!("num = {:?}", num);
    }
}
