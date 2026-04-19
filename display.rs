use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    img: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.img < 0.0 {
            write!(f, "{real} - {img}i", real=self.real, img=self.img.abs())
        } else {
            write!(f, "{real} + {img}i", real=self.real, img=self.img)
        }
        
    }
}

fn main() {
    let s = Structure(67);
    let r = -7.5;
    let i = -6.7;
    let c = Complex { real: r, img: i };
    println!("{}", s);
    println!("{:?}", c);
    println!("{}", c);
}
