use std::collections::HashMap;

// Silver Coins, Ruby
fn main() {
    let mut treasures: HashMap<&str, i32> = HashMap::new();

    treasures.insert("Silver Coins", 100);
    treasures.insert("Ruby", 3);

    if let Some(ruby) = treasures.get_mut("Ruby") {
        *ruby += 5;
    }

    println!("Treasures: {:?}", treasures);
}
