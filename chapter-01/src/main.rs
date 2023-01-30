use std::mem::size_of_val;
use std::collections::HashSet;

fn main() {
    println!("\n** Chapter 01\n");

    print_numbers_v1();
    print_numbers_v2();
    print_numbers_v3();
    print_numbers_v4();

    println!("");
    greet_us();
    greet_us_array();

    foundations_array();
    foundations_vector();
    foundations_set();
}

fn print_numbers_v1() {
    let mut num = 2;
    let mut count = 0;
    let mut count_ifs = 0;

    while num <= 100 {
        if num % 2 == 0 {
            // println!("{num} is even.");
            count_ifs += 1;
        }
        num += 1;
        count += 1;
    }
    println!("print_numbers_v1 requires {count} 'while' and {count_ifs} 'if' calls.");
}

fn print_numbers_v2() {
    let mut num = 2;
    let mut count = 0;

    while num <= 100 {
        // println!("{num} is even.");
        num += 2;
        count += 1;
    }
    println!("print_numbers_v2 requires {count} 'while' calls.");
}

fn print_numbers_v3() {
    let mut count = 0;

    for _num in (2..=100).step_by(2) {
        // println!("{_num} is even");
        count += 1;
    }
    println!("print_numbers_v3 requires {count} 'for' loops.");
}

fn print_numbers_v4() {
    let mut count = 0;

    (2..=100)
        .filter(|n| n % 2 == 0)
        .for_each(|_n| {
            // println!("{_n}");
            count += 1
        });

    println!("print_numbers_v4 requires {count} 'for_each' loops.");
}

fn greet_us() {
    let x = "Hello! ";
    let y = "How are you ";
    let z = "today? ";

    println!("It greets us: {x}{y}{z}");
}

fn greet_us_array() {
    let arr = ["Hello! ", "How are you ", "today? "];

    println!("Array greets us too: {}{}{}", arr[0], arr[1], arr[2]);
}

fn foundations_array() {
    let mut arr = ["apples", "bananas", "dates", "dates"];

    println!("\n*** Array");
    println!("{:?}", arr);
    println!("The size of 'arr' is {}", arr.len());
    println!("'arr' occupies {} bytes", size_of_val(&arr));

    //// Read
    let i = 1;
    if let Some(val) = arr.get(i) {
        println!("Read value at index {}: {:?}", i, val);
    }

    //// Search
    // Linear search.
    let lookup = "dates";
    for (i, val) in arr.iter().enumerate() {
        if val.eq(&lookup) {
            println!("Found {:?} at index {}", val, i);
        }
    }

    //// Insert
    // Kinda 🫣. Array is a fixed-size type in Rust,
    // hence we cannot insert new elements.
    // But we can change value stored at some index.
    arr[1] = "pears";
    println!("'arr' after replace {:?}", arr);

    //// Delete
    // Kinda 🫣. Array is a fixed-size type in Rust.
    // Since we're playing around with the basics,
    // let's mention a dynamically-sized slice here.
    // We also examine a vector (growable array)
    // in the following function.
    let sl = &arr[1..];
    println!("Slice without 'apples': {:?}", sl);
}

fn foundations_vector() {
    let mut vec = vec!["apples", "bananas", "dates", "dates"];

    println!("\n*** Vector");
    println!("{:?}", vec);
    println!("The size of 'vec' is {}", vec.len());
    println!("'vec' occupies {} bytes", size_of_val(&vec));

    //// Read
    let i = 1;
    if let Some(val) = vec.get(i) {
        println!("Read value at index {}: {:?}", i, val);
    }

    //// Search
    // Linear search. Find all occurrences.
    let lookup = "dates";
    for (i, val) in vec.iter().enumerate() {
        if val.eq(&lookup) {
            println!("Found {:?} at index {}", val, i);
        }
    }

    //// Insert
    vec.insert(1, "horses");
    // Append to the back of collection.
    vec.push("pears");
    println!("'vec' after inserts is {:?}", vec);

    //// Delete
    vec.remove(1);
    // Remove the last element.
    vec.pop();
    println!("'vec' after removals is {:?}", vec);
}


fn foundations_set() {
    let mut set = HashSet::from(["apples", "bananas", "dates", "dates"]);

    println!("\n*** HashSet");
    println!("{:?}", set);
    println!("The size of 'set' is {}", set.len());
    println!("'set' occupies {} bytes", size_of_val(&set));

    //// Read
    if let Some(val) = set.get(&"bananas") {
        println!("Got reference to {:?}", val);
    }

    //// Search
    // HashSet does not implement Index trait by default,
    // hence we only check if 'set' contains a value.
    let lookup = "dates";
    println!("'set' contains {:?}: {}", lookup, set.contains(lookup));

    //// Insert
    set.insert("horses");
    set.insert("dates");
    println!("'set' after inserts is {:?}", set);

    //// Delete
    set.remove("horses");
    println!("'set' after removal is {:?}", set);
}
