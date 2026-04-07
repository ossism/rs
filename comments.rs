// This is a single line comment
// This is another one
/*
But this could also be a comment
And it is called a block comment
*/

// Now lets try something else

fn main() {
    // Notice this is unique use of block comments within an expression
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
