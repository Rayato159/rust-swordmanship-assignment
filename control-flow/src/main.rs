// If it’s "sunny", Crabby will cross the river by swimming! 🌞
// If it’s "rainy", Crabby will build a bridge to stay dry. ☔
// If it’s "stormy", Crabby will wait for better weather. ⛈️

// fn main() {
//     let weather = "rainy";

//     if weather == "sunny" {
//         println!("Crabby will cross the river by swimming! 🌞");
//     } else if weather == "rainy" {
//         println!("Crabby will build a bridge to stay dry. ☔");
//     } else {
//         println!("Crabby will wait for better weather. ⛈️");
//     }
// }

// If Crabby encounters a "goblin", he uses his rusty sword to attack. 🦀⚔️
// For a "troll", Crabby sets a trap! 🐾
// If he meets a "dragon", Crabby runs for cover! 🐉🏃‍♂️
// Everything else? Crabby is confused... 😵

// fn main() {
//     let monster = "dragon";

//     match monster {
//         "goblin" => println!("he uses his rusty sword to attack. 🦀⚔️"),
//         "troll" => println!(" Crabby sets a trap! 🐾"),
//         "dragon" => println!("Crabby runs for cover! 🐉🏃‍♂️"),
//         _ => println!("Crabby is confused... 😵"),
//     };
// }

// Crabby needs to build a boat by collecting 10 pieces of wood ! 
// Create a loop that helps Crabby gather wood until he reaches 10 pieces.

// Crabby finished the boat!🛶 

fn main() {
    let mut wood = 0;

    loop {
        wood += 1;

        println!("Crabby gathered {} pieces of wood.", wood);

        if wood == 10 {
            println!("Crabby finished the boat!🛶");
            break;
        }
    }
}