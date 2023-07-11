pub mod if_else {
    pub fn if_else(age: i32) {
        // If is an expression
        // ( ) are optional

        // If else
        if age >= 18 {
            println!("You are an adult");
        } else {
            println!("You are not an adult");
        }

        // If else if with return
        let result: &str = if age >= 18 {
            "You are an adult"
        } else if age >= 13 {
            "You are a teenager"
        } else {
            "You are not an adult"
        };

        // Print result
        println!("{}", result);

        // If else if with return
    }
}
