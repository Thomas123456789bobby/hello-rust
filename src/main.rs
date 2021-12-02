mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod button;


fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();  
    arrays::run();  
    button::run();  
}
let n:i32 = 0;
while n <1000000 {
    n++;
}
 