/*
Primitive types--
interger: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, isize, usize (number of bits they take in memory)
floating point: f32, f64
boolean (bool)
charecters (char)
tuples
arrays
*/

//rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
// Default is "i32"
let x = 1;

//default is "f64"
let y = 2.5;

//add explicit type
let z: i64 = 4545445454545;

//find mex size 
println!("max i32; {}", std::i32::MAX);
println!("max i64; {}", std::i64::MAX);

// Boolean
let is_active: bool  = true;

//get boolean from expression
let is_greater: bool = 10 < 5;

let a1 = 'a';
let b1 = '\u{1F346}';
let b2 = '\u{1F351}';

 println!("{:?}", (x, y, z, is_active, is_greater, a1, b1, b2));
}