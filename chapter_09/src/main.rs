// Starting this chapter, we use more of Rust-specific syntax constructs, such
// as generics, derive attributes, etc.

use linter::Linter;
use printer::PrintManager;

mod linter;
mod printer;

fn main() {
    println!("\n*** Chapter 09 ***\n");

    let code = "( var x = { y: [1, 2, 3] } )";
    println!("Lint: {}", code);
    println!("Result: {:?}\n", Linter::new().lint(code));

    let code = "( var x = { y: [1, 2, 3 })";
    println!("Lint: {}", &code);
    println!("Result: {:?}\n", Linter::new().lint(code));

    let mut print_manager = PrintManager::new();
    print_manager.queue_job("First".to_string());
    print_manager.queue_job("Second".to_string());
    print_manager.queue_job("Third".to_string());
    println!("{:?}", &print_manager);
    print_manager.run();
}
