// Primitive srr = Immutable fexid-length string somewhere in memory
//string = growable, heap-allocated data structure - use when you need to modify of own string data


pub fn run() {
    let mut hello = String::from("hello ");

 // get length
    println!("Length: {}", hello.len());

// pust char
    hello.push('w');

// pust string
    hello.push_str("orld!");

//capacity in bytes
    println!("Capacity: {}", hello.capacity());

// chack if empty
    println!("is empty: {}", hello.is_empty());

//contains
println!("contains 'world' {}", hello.contains("world"));

//replace
println!("replace: {}", hello.replace("world", "there"));

//loop through string by whitespace
for word in hello.split_whitespace() {
    println!("{}", word);
}


//create string with capacity
let mut s = String::with_capacity(10);
s.push('a');    
s.push('b');

//assertion testing
assert_eq!(3, s.len());

    println!("{}", s);


}