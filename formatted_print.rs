fn main() {
    // Automatic stringification
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, meet {1}. {1}, meet {0}.", "Alice", "Bob");

    //Named arguments
    println!(
        "{subject} {verb} {object}",
        subject = "sun",
        verb = "rises",
        object = "in the east."
    );

    // Conversions
    println!("Base 10: {}", 69420);
    println!("Base 2(binary): {:b}", 69420);
    println!("Base 8(octal): {:o}", 69420);
    println!("Base 16(hex): {:x}", 69420);

    // Padding
    println!("{n:>5}", n = 1);
    println!("{n:0<5}", n = 1);
    println!("{n:0>5}", n = 1);

    // Variable padding, with `$`
    println!("{n:0>w$}", n = 1, w = 10);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct S(i32);

    // Unless the struct S implements `fmt::Display`, you cannot do this...
    //println!("This struct `{}` won't print", S(5));

    // From the variables
    let n: f64 = 1.0;
    let w: usize = 5;

    println!("{n:>w$}");

    // Printing pi
    let pi = 3.141592;
    println!("{:.2}", pi);
}
