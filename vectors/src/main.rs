// "Gold", "Silver", "Ruby Gem", "Emerald"
fn main() {
    let mut items = vec!["Gold", "Silver", "Ruby Gem"];

    items.remove(1);
    items.push("Diamond");

    println!("Items: {:?}", items);
    println!("Items length: {}", items.len());
    println!("Items capacity: {}", items.capacity());
}
