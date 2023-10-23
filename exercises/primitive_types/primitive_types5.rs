// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let cat = ("Furry McFurson", 3.5);
    // we can use syntax below to assign a bunch of variables 
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
