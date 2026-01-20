mod coins;
mod greedy;
mod shelf;
mod sum;
mod swap;

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

    let n = 10;
    println!("\nYou win with {} coins: {}", n, coins::is_winning_1(n));

    let n = 9;
    println!("You win with {} coins: {}", n, coins::is_winning_1(n));

    let n = 10;
    println!("\nYou win with {} coins: {}", n, coins::is_winning_2(n));

    let n = 9;
    println!("You win with {} coins: {}", n, coins::is_winning_2(n));

    let a1 = [5, 3, 2, 9, 1];
    let a2 = [1, 12, 5];
    println!("\na1: {:?}", &a1);
    println!("a2: {:?}", &a2);
    println!(
        "Indices to swap: {:?}",
        swap::find_indices_1(&a1, &a2).unwrap()
    );

    let a1 = [5, 3, 2, 9, 1];
    let a2 = [1, 12, 5];
    println!("\na1: {:?}", &a1);
    println!("a2: {:?}", &a2);
    println!(
        "Indices to swap: {:?}",
        swap::find_indices_2(&a1, &a2).unwrap()
    );

    // max(): See chapter_04/src/exercises.rs greatest_number()

    let d = [3, -4, 4, -3, 5, -9];
    println!("\ndata: {:?}", &d);
    println!("Largest subsection sum: {:?}", greedy::max_segment_sum(&d));

    let d = [22, 25, 21, 18, 19, 17, 16, 20];
    println!("\ndata: {:?}", &d);
    println!("Upward trend: {:?}", greedy::upward_trend(&d));

    let d = [50, 51, 48, 49, 47, 48, 46];
    println!("\ndata: {:?}", &d);
    println!("Upward trend: {:?}", greedy::upward_trend(&d));
}
