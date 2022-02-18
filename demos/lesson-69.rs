fn main() {
    let a:Option<i32> = Some(1);
    dbg!(a);

    let a_is_some = a.is_some();
    dbg!(a);

    let a_is_none_ = a.is_none();
    let a_mapped = a.map(|num| num + 1);
    let a_filtered = a.filter(|num| num == &1);
    let a_or_else = a.or_else(|| Some(5));
    let unwrapped = a.unwrap_or_else(|| 0);
}
