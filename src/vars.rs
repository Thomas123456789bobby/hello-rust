// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "thomas";
    let mut age = 16;
    println!("My name is {} and my age is {}", name, age);
    age = 17;
    println!("My name is {} and my age is {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Thomas", 16);
    println!("{} is {}", my_name, my_age);

}