#![allow(clippy::four_forward_slashes)]

mod average_celsius;
mod average_evens;
mod clothing;
mod count_ones;
mod exercises;
mod palindrome_checker;
mod password_cracker;
mod products;
mod sample;
mod word_builder;

fn main() {
    println!("\n*** Chapter 07 ***\n");

    let v = vec![4, 2, 4, 1, 3];
    println!("Vector: {:?}", v);
    println!("Evens average: {:?}\n", average_evens::find(&v).unwrap());

    let v = vec!['a', 'b', 'c', 'd'];
    println!("Vector: {:?}", v);
    println!("Permutations: {:?}\n", word_builder::build(&v));

    let v = vec![2, 5, 8, 10, 12, 11];
    println!("Vector: {:?}", v);
    println!("Sample: {:?}\n", sample::sample(&v).unwrap());

    let v = vec![100.5, 80.0, 40.2, 33.3];
    println!("Fahrenheit readings: {:?}", v);
    println!(
        "Celsius average: {:?}\n",
        average_celsius::find(&v).unwrap()
    );

    let v = vec!["Skirt", "Dress"];
    println!("Clothes: {:?}", v);
    println!("Inventory: {:?}\n", clothing::mark_inventory(&v, 5));

    let v = vec![vec![0, 1, 1, 1, 0], vec![0, 1, 0, 1, 0, 1], vec![1, 0]];
    println!("Vector: {:?}", v);
    println!("Ones: {:?}\n", count_ones::count(v));

    let s = "rötör";
    println!("String: {:?}", s);
    println!("Palindrome: {:?}\n", palindrome_checker::is_palindrome(s));

    let v = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", v);
    println!("Products: {:?}\n", products::get(&v));

    let v1 = vec![1, 2, 3];
    let v2 = vec![10, 100, 1000];
    println!("Vector 1: {:?}", v1);
    println!("Vector 2: {:?}", v2);
    println!("Products: {:?}\n", products::get_multi(&v1, &v2));

    println!(
        "Password Combinations Iterative: {:?}\n",
        password_cracker::combinations(3)
    );
    println!(
        "Passwords Combinations Recursive: {:?}\n",
        password_cracker::combinations_recur(3)
    );

    //// Exercises
    println!("\n*** Exercises ***\n");

    let v = vec![1, 97, 3, 2];
    let is_100_sum = exercises::is_one_hundred_sum(&v);
    println!("{:?} contains a sum 100: {}\n", &v, is_100_sum);

    let a = vec![1, 97];
    let b = vec![-3, 2, 103];
    let merged = exercises::merge_sorted(&a, &b);
    println!("{:?} and {:?} merged to {:?}\n", &a, &b, merged);

    let needle = "dog";
    let haystack = "hotdogs";
    println!("Found '{}' in a '{}': {}\n",
             needle,
             haystack,
             exercises::find_needle(needle, haystack));

    let v = vec![0, -42, -1, 1];
    println!("The largest product in '{:?}' is '{:?}'\n",
             v,
             exercises::largest_product(&v));

    let v = vec![50, 40, 60, 70, 10, 100];
    println!("Picked resume '{:?}' from '{:?}'\n",
             exercises::pick_resume(&v).unwrap(),
             &v);
}
