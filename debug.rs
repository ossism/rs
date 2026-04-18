#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct DeepStructure(Structure);

#[derive(Debug)]
struct Person <'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let s = Structure(67);
    let d = DeepStructure(Structure(61));
    let name = "sam";
    let age = 50;
    let p = Person { name, age };

    println!("{:?}", s);
    println!("{:?}", d);

    // Doing a pretty debug print using `{:#?}`. Notice the `#` :)?

    println!("{} {:p}", name, &name);
    println!("{} {:p}", p.name, &p.name);
    println!("{:#?}", p);
}
