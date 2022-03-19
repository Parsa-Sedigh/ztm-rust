// Topic: Arrays & Slices
//
// Requirements:
// * Print pairs of numbers and their sums as they are streamed from a data source
// * If only one number is received, then print "Unpaired value: V",
//   where V is the value
// * If no numbers are received, print "Data stream complete"
//
// Notes:
// * A simulated data stream is already configured in the code
// * See the stdlib docs for the "chunks" method on "slice" for more info

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

fn process_chunk(data: &[u64]) {
    match data {
        [lhs, rhs] => println!("{}+{}={}", lhs, rhs, (lhs + rhs)),
        [single] => println!("Unpaired value: {}", single),
        [] => println!("Data stream complete"), // if we would have an even number of values, then the this match arm would be triggered
        /* this would terminate the program if it was executed, because the data argument size should always be
        either 2, 1 or an empty slice. So if we get a slice that has 10 elements in data argument, the code will be terminated.*/
        [..] => unreachable!("chunk size should be at most 2")
    }
}

fn main() {
    // `stream` is an iterator of Option<&[u64]>
    let mut stream = data().chunks(2);
    for chunk in stream {
        process_chunk(chunk);
    }
}
