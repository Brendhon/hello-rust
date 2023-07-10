pub mod constants {
    pub fn constants() {
        // Can be declared in any scope
        // Constant - Is not save in the memory - Is replaced by the value on compile time
        const PI: f32 = 3.1415927;

        // Show the value of PI
        println!("value of PI is {}", PI);
    }
}
