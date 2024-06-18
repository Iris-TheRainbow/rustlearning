 #[allow(dead_code)] 
 fn main(){
    println!("{} days", 31);
    println!("{0}, this is {1}. {0}, this is {1}", "Alyce", "Bob");
    println!("{Subject} {Verb} {Object}",
        Object = "the lazy dog",
        Subject = "the quick brown fox",
        Verb = "jumped over");
    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("base 8 (octal): {:o}", 69420);
    println!("base 16 (hexidecimal): {:x}", 69420);
    println!("{number:>5}", number=1);
    println!("{number:<5}", number=1);
    println!("My name is {0}, {1} {0}", "james", "bond");
    struct Structure(i32);
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
 }