enum CrabbyStates {
    Fighting,
    Collecting(u32),
    Defending,
}

impl CrabbyStates {
    fn state_represent(&self) {
        match self {
            CrabbyStates::Fighting => println!("Crabby is fighting"),
            CrabbyStates::Collecting(amount) => println!("Crabby is collecting {}", amount),
            CrabbyStates::Defending => println!("Crabby is defending"),
        }
    }
}

fn main() {
    let fighting = CrabbyStates::Fighting;
    let collecting = CrabbyStates::Collecting(15);
    let defending = CrabbyStates::Defending;

    fighting.state_represent();
    collecting.state_represent();
    defending.state_represent();
}
