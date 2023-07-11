pub mod mutable {
    pub fn mutable() {
        // Mutable variable
        let mut mutable_variable: i32 = 5;

        // Show the value of mutable_variable
        println!("value of mutable_variable is {}", mutable_variable);

        // Update the value of mutable_variable
        mutable_variable = 10;

        // Show the value of mutable_variable
        println!("value of mutable_variable is {}", mutable_variable);

        // Immutable variable (By default all variables are immutable)
        let immutable_variable: i32 = 5;

        // Show the value of immutable_variable
        println!("value of immutable_variable is {}", immutable_variable);

        // Update the value of immutable_variable (This will throw an error)
        // immutable_variable = 10;
    }
}
