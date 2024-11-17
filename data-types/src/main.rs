fn main() {
    let x = 1;
    let y = 0.5;

    let z = x + y as i32;

    let msg = String::from("Hello, World!");
    let msg2 = "Hello, World!".to_string();
    let msg3 = "Hello, World!";
    let msg4 = format!("Point: {}, {}", x, y);

    println!("{}", msg4);
}
