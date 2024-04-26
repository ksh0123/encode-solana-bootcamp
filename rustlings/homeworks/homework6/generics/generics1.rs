// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

fn main() {
    let mut shopping_list: Vec<String> = Vec::new();
    shopping_list.push("milk".to_string());
    // .to_string() creates a copy of "milk".
}

// There are two ways to use generics here:

// Vec<String> | "milk".to_string()
// Storing the `String` object
// This method is used when you need ownership of the string
// and the strings should be mutable.

// Vec<&str> | "milk"
// Storing the `&str` slice
// This method is used when you know the items are static
// and it doesn't matter if the items are read-only.
// The string literals can be pushed to the vector.