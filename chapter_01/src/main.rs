mod foundations;
mod greet_us;
mod print_numbers;

fn main() {
    println!("\n*** Chapter 01 ***\n");

    print_numbers::evens_v1();
    print_numbers::evens_v2();
    print_numbers::evens_step_by();
    print_numbers::evens_filter();

    println!();
    greet_us::hello_strings();
    greet_us::hello_array();

    foundations::review_array();
    foundations::review_vector();
    foundations::review_hashset();
    foundations::review_btreeset();
}
