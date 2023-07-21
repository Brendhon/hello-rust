pub mod references {
    pub fn references() {
                // String in rust is a pointer to a memory space
        // This happens because the size of a String is unknown at compile time and can change

        // Save 'hello' in the memory space and save the pointer in the variable s
        // 's' is the owner of the String
        let mut s: String = String::from("hello");

        // Pass the ownership of the String to the function and return the ownership of the String
        // 'bad smell' becase if you whant to use the variable s after the function call you need to return the ownership of the String
        s = show_and_return(s);

        // Now we can use the variable s because the ownership of the String was returned to the variable
        println!("s: {}", s);

        // BORROWING the ownership of the String
        show(&s);

        // Now we can use the variable s without need to return the ownership of the String
        println!("s: {}", s);

        // BORROWING the mutable ownership of the String - the function can change the String
        // It's only possible to borrow the mutable ownership of the String if the variable is mutable
        show_mut(&mut s);

        // Show the String after the function call - the function changed the String
        println!("s: {}", s);
    }

    // Print the String - the function is the owner of the String
    fn show_and_return(s: String) -> String {
        println!("Show and return: {}", s);
        s
    }

    // Pass the reference of the String to the function
    fn show(s: &String) {
        println!("Show: {}", s);
    }

    // Pass the mutable reference of the String to the function
    fn show_mut(s: &mut String) {
        // Change the String
        s.push_str(" world");

        // Show new String
        println!("Show mut: {}", s);
    }
}
