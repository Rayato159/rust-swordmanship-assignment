// Create a module for potions
use modules_and_crates::potions::use_item;

// Create a module for weapons
mod weapons {
    pub fn use_item() {
        println!("You used a weapon!");
    }
}

// Create a module for maps
mod maps {
    pub fn use_item() {
        println!("You used a map!");
    }
}

fn main() {
    // Use each tool here!
    use_item::use_item();
    weapons::use_item();
    maps::use_item();
}
