use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    let _slice = &map.clone().into_iter().collect::<Vec<_>>()[..];

    println!("{:?}", _slice);

    map.clear()
}
