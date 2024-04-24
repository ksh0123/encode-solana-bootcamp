fn main(){
    // unsigned integer
    // u8. u16, u32. u64, u128
    let unsigned: u8 = 10;
    // signed integer
    // i8, i16, i32, i64, i128
    let signed: i8 = -10;
    // float is used for decimals
    let float: f32 = 1.2;
    println!("unsign: {} sign: {} float: {}", unsigned, signed, float);


    // char - can only be
    let letter = "c1234";
    let emoji = "\u{1F600}"; 
    println!("letter: {} emoji: {}", letter, emoji);

    // boolean
    let is_true: bool = true;
    
    println!("isTrue: {}", is_true);


    // array
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {} length: {}", arr[0], other_arr.len());
    // print structure of array and other objects
    println!("{:?}", other_arr);


    // tuples
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5, "hello");


    // print structure of tupple and other objects
    println!("first {} second {} third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    let (a, b, c) = tuple2;

    // destructuring
    println!("first {} second {} third {}", a, b, c);
    // functions - all rust functions are private unless declared `pub`
    println!("Is {} an even number? {}", 2, is_even(2));


    // Variable are immutable by default 
    // Use the keyword `mut` to make the variable mutable
    let mut num = 5;
    num = 3;
    println!("Num is {}", num);


    // Arrays & Slices
    // Array lengths are known at compile time
    let arr1 = [0, 1, 2, 3];
    // first value is inclusive, second value is exclusive
    // Slice length is not known at compile time
    let slice = &arr1[1 .. 3];
    borrowing_slice(arr1, slice);


    // Strings
    let str: &str = "hello world"; // `&` reference to the space in memory that holds the constant holding "Hello world"
    let mut string: String = String::from("Hello world"); // String object type which is equivalent to a vector that you can keep adding to
    let str_slice = &string[.. 6]; // Must use mut keyword to modify
    slice.len();

    string.push('1'); // add character
    string.push_str("! Bob"); 
    string = string.replace("Hello", "Bye");
    println!("{}", string);


    // If Statements
    let n = 3;
    if n > 0 {
        println!("Greater than 0");
    } else if n < 0 {
        println!("Less than 0");
    } else {
        println!("is 0");
    }
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return bool
}

// helper function
fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}