pub mod errors {
  pub fn errors() {
    // Vector
    let v: Vec<i32> = vec![1, 2, 3];

    // Accessing an element out of the vector | panic
    // v[4];

    // Accessing an element out of the vector | Option
    match v.get(4) {
      Some(value) => println!("Value: {}", value),
      None => println!("None"),
    }

    // Panic -- Rust will print a message and exit
    // panic!("Panic");

    // Result - Success return string | Error return u8
    let result: Result<String, u8> = Ok(String::from("OK"));

    // Match result
    match result {
      Ok(value) => println!("Value: {}", value),
      Err(value) => println!("Error: {}", value),
    }
  }
}
