struct Crabby {
    name: String,
    health: u8, // 100
}

impl Crabby {
    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal >= 100 {
            self.health = 100;
            return;
        }
        self.health += heal;
    }
}

fn main() {
    let mut crabby = Crabby {
        name: "Crabby".to_string(),
        health: 100,
    };

    crabby.take_damage(100);
    crabby.take_damage(10);
    println!("Crabby health: {}", crabby.health);

    crabby.healing(60);
    println!("Crabby health: {}", crabby.health);
}
