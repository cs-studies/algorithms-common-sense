mod shelf;
mod sum;

fn main() {
    println!("\n*** Chapter 20 ***\n");

    let authors = shelf::sample::authors();
    println!("{:#?}", authors);

    let books = shelf::sample::books();
    println!("{:#?}", books);

    let connected = shelf::books_with_authors_1(&books, &authors);
    println!("{:#?}", connected);

    let connected = shelf::books_with_authors_2(&books, &authors);
    println!("{:#?}", connected);

    let a = &[2, 0, 4, 1, 7, 9];
    println!("\ndata: {:?}", a);
    println!("Found numbers that add up to 10: {}", sum::two_sum_1(a));

    let a = &[2, 0, 4, 1, 7, 9];
    println!("\ndata: {:?}", a);
    println!("Found numbers that add up to 10: {}", sum::two_sum_2(a));
}
