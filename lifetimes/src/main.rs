fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}
fn main() {
    let map1 = "Ancient Map of the Sea";
    let map2 = "Map to Hidden Gold";

    let chosen_map = longest_map(map1, map2);

    println!("Crabby's longest map: {}", chosen_map);
}
