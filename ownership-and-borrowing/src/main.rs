fn main() {
    let mut treasure = String::from("gold coins");
    // Multiple friends borrow immutably!
    // code here ...

    let friend1 = &treasure;
    let friend2 = &treasure;

    println!("Friend 1 sees: {}", friend1);
    println!("Friend 2 sees: {}", friend2);

    // Trusted friend borrows mutably
    // code here ...

    let trusted_friend = &mut treasure;

    trusted_friend.push_str(" and silver coins");
    println!("Trusted friend updates: {}", trusted_friend);
}
