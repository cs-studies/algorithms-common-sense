mod anagrams;
mod coins;
mod exercise1;
mod exercise2;
mod exercise3;
mod greedy;
mod group;
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

    let s1 = "rattles";
    let s2 = "startle";
    println!("\ns1: {s1}");
    println!("s2: {s2}");
    println!("are anagrams: {}", anagrams::are_anagrams_1(s1, s2));
    println!("are anagrams: {}", anagrams::are_anagrams_2(s1, s2));

    let d = ['a', 'c', 'd', 'b', 'b', 'c', 'a', 'd', 'c', 'b', 'a', 'd'];
    println!("\narray: {:?}", &d);
    println!("grouped: {:?}", group::group_array(&d));

    println!("\n*** Exercises ***\n");

    let basketball_players = exercise1::sample::basketball_players();
    let football_players = exercise1::sample::football_players();
    println!(
        "Same players: {:#?}",
        exercise1::find_same_players(&basketball_players, &football_players)
    );

    let d = [2, 3, 0, 6, 1, 5];
    println!("\narray: {:?}", d);
    println!("Missing number: {:?}", exercise2::find_missing_number_1(&d));
    let d = [8, 2, 3, 9, 4, 7, 5, 0, 6];
    println!("\narray: {:?}", d);
    println!("Missing number: {:?}", exercise2::find_missing_number_1(&d));

    let d = [2, 3, 0, 6, 1, 5];
    println!("\narray: {:?}", d);
    println!("Missing number: {:?}", exercise2::find_missing_number_2(&d));
    let d = [8, 2, 3, 9, 4, 7, 5, 0, 6];
    println!("\narray: {:?}", d);
    println!("Missing number: {:?}", exercise2::find_missing_number_2(&d));

    let d = [10, 7, 5, 8, 11, 2, 6];
    println!("\nprices: {:?}", d);
    println!("Greatest profit: {:?}", exercise3::greatest_profit(&d));
}
