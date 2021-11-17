// Primitive srr = Immutable fexid-length string somewhere in memory
//string = growable, heap-allocated data structure - use when you need to modify of own string data


pub fn run() {
    let hello = String::from("hello");

    println!("{}", hello);

}