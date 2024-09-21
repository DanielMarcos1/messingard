pub mod chapter_2 {
    // Types and stuff, we have scalar and compound types

    // Scalar types: integers, floats, booleans, and characters
    pub fn integer() {
        let integer: i32 = 1; // Integers can have sizes of i32, i64, i128, u32, u64, u128, usize, isize 
                              // i = signed, u = unsigned
        println!("Here's an integer: {}", integer);
    }

    pub fn float() {
        let float: f64 = 21.48; // Floats can have sizes of f32, f64
        println!("Here's a float: {}", float);
    }

    pub fn boolean() {
        let boolean: bool = true; // Booleans can be true or false
        println!("Here's a boolean: {}", boolean);
    }

    pub fn character() {
        let character: char = 'a'; // Characters can be a single character
        println!("Here's a character: {}", character);
    }

    // Compound types: tuples, arrays, and structs
    pub fn array() {
        let array: [i32; 5] = [1, 2, 3, 4, 5]; // Arrays can be of any type
        println!("Here's an array: {:?}", array);
    }

    pub fn tuple() {
        let tuple: (i32, f64, bool, char) = (1, 2.0, true, 'a'); // Tuples can be of any type
        println!("Here's a tuple: {:?}", tuple);
    }

    pub fn parentheses_type() {
        let _parentheses_type: () = (); // This type is used for functions that don't return anything
                                       // the return is an empty tuple
    }

    // Remember that variables that uses these tipes can be mut or not    
}