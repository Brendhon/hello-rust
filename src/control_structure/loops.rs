pub mod loops {
    pub fn loops(multiplication_table: u8) {
        // Counter
        let mut counter: u8 = 0;
        const LIMIT: u8 = 10;

        // While loop
        println!("----- While loop -----");
        while counter < LIMIT {
            // Increment counter
            counter += 1;

            // Continue if counter is 5 (skip 5)
            if counter == 5 {
                continue;
            }

            // Print result
            println!(
                "{} x {} = {}",
                multiplication_table,
                counter,
                multiplication_table * counter
            );
        }

        // Break line
        println!();

        // Reset counter
        counter = 0;

        // loop
        println!("----- loop -----");
        loop {
            if counter == LIMIT {
                break;
            }

            // Increment counter
            counter += 1;

            // Print result
            println!(
                "{} x {} = {}",
                multiplication_table,
                counter,
                multiplication_table * counter
            );
        }

        // Break line
        println!();

        // For loop
        println!("----- For loop -----");
        for i in 0..=LIMIT { // 0..=LIMIT is a range from 0 to LIMIT (inclusive)
            // Print result
            println!(
                "{} x {} = {}",
                multiplication_table,
                i,
                multiplication_table * i
            );
        }
    }
}
