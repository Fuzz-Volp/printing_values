fn main() {
    println!("Hello, world!");

    // Formatting
    println!("My name is {} and I'm {} years old", "John", "27");

    // Expressions
    println!("a + b = {}", 3 + 9);

    // Positional Arguments
    println!("{0} has a {2} and {0} has a {1}", "John", "cat", "dog");

    // Named Arguments
    println!("{name} {surname}", surname="Volpitta", name="John");

    // Printing Traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50);

    // Debug
    println!("Array: {:?}", [1, 2, 3]);
}
