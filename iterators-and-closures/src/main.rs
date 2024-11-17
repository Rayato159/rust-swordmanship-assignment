fn main() {
    let treasures = vec![100, 200, 300, 400];
    // Code here ...

    let doubled_treasures: Vec<i32> = treasures.iter().map(|x| x * 2).collect();

    println!("{:?}", doubled_treasures);
}
