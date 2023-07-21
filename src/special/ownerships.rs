pub mod ownerships {
  pub fn ownerships() {
    // String in rust is a pointer to a memory space
    // This happens because the size of a String is unknown at compile time and can change

    // Save 'hello' in the memory space and save the pointer in the variable s
    // 's' is the owner of the String
    let mut s: String = String::from("hello");

    // Push a string into the String
    s.push_str(", world!");

    // When pass by parameter, the ownership of the String is passed to the function
    // The function will be the owner of the String
    show(s);

    // We can't use the variable s anymore because the ownership of the String was passed to the function
    // println!("{}", s);
  }

  // Print the String - the function is the owner of the String
  fn show (s: String) {
    println!("{}", s);
  }
}
