// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

use delicious_snacks::fruits::PEAR as fruit;
use delicious_snacks::veggies::CUCUMBER as veggie;

fn main() {
    println!(
        "favorite snacks: {} and {}",
        fruit,
        veggie,
    );
}
