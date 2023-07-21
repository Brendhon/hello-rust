// Modules to import
mod syntaxes;
mod control_structure;
mod special;

// Syntaxes
use crate::syntaxes::constants::constants::constants;
use crate::syntaxes::memory_size::memory_size::memory_size;
use crate::syntaxes::mutable::mutable::mutable;
use crate::syntaxes::statics::statics::statics;
use crate::syntaxes::types::types::types;
use crate::syntaxes::functions::functions::sum;

// Control structure
use crate::control_structure::if_else::if_else::if_else;
use crate::control_structure::loops::loops::loops;
use crate::control_structure::matchs::matchs::matchs;

// Special
use crate::special::ownerships::ownerships::ownerships;
use crate::special::references::references::references;
use crate::special::pattern_matching::pattern_matching::pattern_matching;

fn main() {
    // ----- Syntaxes -----

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

    // Break line
    println!();

    // ----- Control structure -----

    // ----- If else -----
    println!("----- If else -----");
    if_else(17);

    // Break line
    println!();

    // ----- Loops -----
    println!("----- Loops -----");
    loops(5);

    // Break line
    println!();

    // ----- Match -----
    println!("----- Match -----");
    matchs("TypeScript");

    // Break line
    println!();

    // ----- Special -----
    println!("----- Special -----");
    ownerships();

    // Break line
    println!();

    // ----- References -----
    println!("----- References -----");
    references();

    // Break line
    println!();

    // ----- Pattern matching -----
    println!("----- Pattern matching -----");
    pattern_matching();
}
