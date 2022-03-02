// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Names<'a> {
    inner: Vec<&'a str>
}

struct Titles<'a> {
    inner: Vec<&'a str>
}

fn main() {
    // we're only interested in the actual DATA and not the name of columns which are in the first line, so we skip that line
    // we don't care what type is the elements of vector, so we just let to figure it out on it's own, so we specify the type as: Vec<_>
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data.iter().filter_map(|line| line.split(',').nth(1)).collect();
    let titles: Vec<_> = data.iter().filter_map(|line| line.split(',').nth(4)).collect();

    let names = Names {inner: names};
    let titles = Titles {inner: titles};

    // for n in names.iter().take(3) {
    //     println!("{}", n);
    // }

    let data = names.inner.iter().zip(titles.inner.iter());
    for (name, title) in data.take(5) {
        println!("Name: {}, title: {}", name, title);
    }
}
