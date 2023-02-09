use crate::foundations::*;
use crate::greet_us::*;
use crate::print_numbers::*;

mod foundations;
mod greet_us;
mod print_numbers;

fn main() {
    println!("\n*** Chapter 01 ***\n");

    evens_v1();
    evens_v2();
    evens_step_by();
    evens_filter();

    println!();
    hello_strings();
    hello_array();

    review_array();
    review_vector();
    review_hashset();
    review_btreeset();
}
