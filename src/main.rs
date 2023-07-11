// Modules
mod constants;
mod memory_size;
mod mutable;
mod statics;
mod types;
mod functions;

// Import functions
use crate::constants::constants::constants;
use crate::memory_size::memory_size::memory_size;
use crate::mutable::mutable::mutable;
use crate::statics::statics::statics;
use crate::types::types::types;
use crate::functions::functions::sum;

fn main() {
    // ----- Types -----
    println!("----- Types -----");
    types();

    // Break line
    println!();

    // ----- Mutable -----
    println!("----- Mutable -----");
    mutable();

    // Break line
    println!();

    // ----- Memory allocation -----
    println!("----- Memory allocation -----");
    memory_size();

    // Break line
    println!();

    // ----- Constants -----
    println!("----- Constants -----");
    constants();

    // Break line
    println!();

    // ----- Statics -----
    println!("----- Statics -----");
    statics();

    // Break line
    println!();

    // ----- Functions -----
    println!("----- Functions -----");
    // Print result
    println!("Sum: {}", sum(1, 3));
}
