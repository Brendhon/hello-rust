pub mod pattern_matching {
  // Doc - https://doc.rust-lang.org/book/ch18-00-patterns.html
  pub fn pattern_matching() {
    for i in 1..=15 {

      // Pattern matching 
      // Check if the value is equal to the value in the match, for example:
      // if i == 1
      // if i == 2 ...
      match i {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        6 => println!("Six"),
        7 => println!("Seven"),
        8 => println!("Eight"),
        9 => println!("Nine"),
        10 => println!("Ten"),
        _ => (),
      }

      // Pattern matching with range | or
      // Check if the value is in the range, for example:
      // if i >= 1 && i <= 5
      // if i >= 6 && i <= 10 ...
      match i {
        1 | 2 => println!("One or two"), // Or operator
        3..=5 => println!("Three to five"), // Range operator
        _ if i % 2 == 0 => println!("Even"), // If operator
        _ => println!("Odd"), // Default
      }
    }

    // Macth point
    let point = (0, 1);

    // Other example of match point
    match point {
      (0, 0) => println!("Origin"),
      (0, y) => println!("x axis, y = {}", y),
      (x, 0) => println!("y axis, x = {}", x),
      (x, y) => println!("({}, {})", x, y),
    }

  }
}
