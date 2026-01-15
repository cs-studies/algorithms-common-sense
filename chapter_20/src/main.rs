mod shelf;

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
}
