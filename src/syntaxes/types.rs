pub mod types {
    pub fn types() {
        // Valid integer types
        // let :i8 = 128; // valid range: -128 to 127
        // let :u8 = -1; // valid range: 0 to 255
        // let :i16 = 32768; // valid range: -32768 to 32767
        // let :u16 = -1; // valid range: 0 to 65535
        // let :i32 = 2147483648; // valid range: -2147483648 to 2147483647
        // let :u32 = -1; // valid range: 0 to 4294967295
        // let :i64 = 9223372036854775808; // valid range: -9223372036854775808 to 9223372036854775807
        // let :u64 = -1; // valid range: 0 to 18446744073709551615
        // let :i128 = 170141183460469231731687303715884105728; // valid range: -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
        // let :u128 = -1; // valid range: 0 to 340282366920938463463374607431768211455
        // let :isize = 9223372036854775808; // valid range: -9223372036854775808 to 9223372036854775807
        // let :usize = -1; // valid range: 0 to 18446744073709551615

        // let integer: i8 = 127;
        let integer: i8 = 127;

        // Show the value of integer
        println!("value of integer is {}", integer);

        // Valid float types
        // let :f32 = 3.40282347E+38; // valid range: 1.17549435E-38 to 3.40282347E+38
        // let :f64 = 1.7976931348623157E+308; // valid range: 2.2250738585072014E-308 to 1.7976931348623157E+308

        // let float: f32 = 2,5;
        let float: f32 = 2.5;

        // Show the value of float
        println!("value of float is {}", float);

        // Valid boolean types
        // let :bool = 0; // valid values: true or false

        // let boolean: bool = 1;
        let boolean: bool = true;

        // Show the value of boolean
        println!("value of boolean is {}", boolean);

        // Valid character types
        // let :char = 0; // valid values: 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j' ... 'z'

        // let character: char = 1;
        let character: char = 'a';

        // Show the value of character
        println!("value of character is {}", character);

        // Valid string types
        // let :str = 0; // valid values "Hello World"...

        // let string: str = 1;
        let string: &str = "Hello World";

        // Show the value of string
        println!("value of string is {}", string);

        // Valid array types
        // let :[i32; 5] = [1, 2, 3, 4, 5]; // valid values: [1, 2, 3, 4, 5]

        // let array: [i32; 5] = [1, 2, 3, 4, 5];
        let array: [i32; 5] = [1, 2, 3, 4, 5];

        // Show the value of array
        println!("value of array is {:?}", array);
    }
}
