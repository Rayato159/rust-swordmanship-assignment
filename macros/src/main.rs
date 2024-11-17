// Create a module for potions
macro_rules! magic_spelling {
    // Code here…
    (fire) => {
        println!("FIRE!");
    };
    (water) => {
        println!("WATER!");
    };
}

fn main() {
    // Crabby Spelling
    magic_spelling!(fire);
    magic_spelling!(water);
}
