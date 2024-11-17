use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let crabby_gold = Arc::new(Mutex::new(10));

    let loot_1 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    let loot_2 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 200;
        }
    });

    let loot_3 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 80;
        }
    });

    loot_1.join().unwrap();
    loot_2.join().unwrap();
    loot_3.join().unwrap();

    println!("Gold: {}", crabby_gold.lock().unwrap());
}
