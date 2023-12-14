// Starting this chapter, we use more of Rust-specific syntax constructs, such
// as generics, derive attribute, etc.

use linter::Linter;

mod linter;

fn main() {
    println!("\n*** Chapter 09 ***\n");

    let code = "( var x = { y: [1, 2, 3] } )";
    println!("Lint: {}", code);
    println!("Result: {:?}\n", Linter::new().lint(code));

    let code = "( var x = { y: [1, 2, 3 })";
    println!("Lint: {}", &code);
    println!("Result: {:?}", Linter::new().lint(code));
}
