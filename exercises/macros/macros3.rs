// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.


mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    pub fn not_a_macro() {
        println!("Bruh");
    }
}

mod serving {
    pub fn deneme() {
        println!("Served");
    }
}

fn main() {
    my_macro!();
    serving::deneme();
    macros::not_a_macro();
}
