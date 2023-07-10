pub mod statics {
    pub fn statics() {
        // Static - Is similar to constants but has a memory allocation
        // Can be declared in any scope
        // Can be mutable

        // Static variable
        static STATIC_VARIABLE: i32 = 5;

        // Show the value of STATIC_VARIABLE
        println!("value of STATIC_VARIABLE is {}", STATIC_VARIABLE);

        // Show memory allocation of STATIC_VARIABLE
        println!(
            "memory allocation of STATIC_VARIABLE is {}",
            std::mem::size_of_val(&STATIC_VARIABLE)
        );
    }
}
