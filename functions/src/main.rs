fn main() {
    let result = crabby_tasks("gathering coins", 12);
    println!("{}", result);
    // Add more function calls here!
    // Then, print the result of each function call

    let result_2 = crabby_tasks("cooking", 30);
    println!("{}", result_2);

    let result_3 = crabby_tasks("hunting", 60);
    println!("{}", result_3);
}

fn crabby_tasks(task: &str, time: i32) -> String {
    format!("Crabby has successfully done {} in {} minutes!", task, time)
}
