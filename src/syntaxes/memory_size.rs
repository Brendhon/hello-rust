pub mod memory_size {
    pub fn memory_size() {
        let integer: i8 = 127;
        let float: f32 = 2.5;
        let boolean: bool = true;
        let character: char = 'a';
        let string: &str = "Hello World";
        let array: [i32; 5] = [1, 2, 3, 4, 5];

        // Show the memory allocation of integer
        println!(
            "memory allocation of integer is {}",
            std::mem::size_of_val(&integer)
        );

        // Show the memory allocation of float
        println!(
            "memory allocation of float is {}",
            std::mem::size_of_val(&float)
        );

        // Show the memory allocation of boolean
        println!(
            "memory allocation of boolean is {}",
            std::mem::size_of_val(&boolean)
        );

        // Show the memory allocation of character
        println!(
            "memory allocation of character is {}",
            std::mem::size_of_val(&character)
        );

        // Show the memory allocation of string
        println!(
            "memory allocation of string is {}",
            std::mem::size_of_val(&string)
        );

        // Show the memory allocation of array
        println!(
            "memory allocation of array is {}",
            std::mem::size_of_val(&array)
        );
    }
}
