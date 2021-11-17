pub fn run(){
    // print to console
    println!("Hello, world! vorm print.rs");

    // basic formatting
    println!("{} komt uit: {}", "thomas" , "obdam");

    // positional arguments
    println!("{0} komt uit {1} en heet {0} en vind het leuk om te {2}"
    , "thomas", "obdam" ,"code"
);

    // named arguments
    println!("{name} komt uit {place} en heet {name} en vind het leuk om te {activity}"
    , name = "thomas" , place = "obdam" , activity = "code");

    // placeholder traits
    println!("Binary: {:b} hex: {:x} octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}