fn main() {
    let treasures = ["Gold", "Silver", "Ruby Gem", "Emerald"];
    let mut energy = 5;

    for treasure in treasures.iter() {
        if energy == 0 {
            println!("You are out of energy. Game over!");
            break;
        } else if treasure == &"Ruby Gem" {
            println!("You found the Ruby Gem! You win!");
            break;
        }

        energy -= 1;
    }
}
