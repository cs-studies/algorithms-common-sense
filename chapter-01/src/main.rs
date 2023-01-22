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
    let mut arr = ["apples", "bananas", "dates"];
    let arr_len = arr.len();

    println!("\n*** Array");
    println!("The size of 'arr' is {}", arr_len);
    println!("'arr' occupies {} bytes", size_of_val(&arr));

    // Read
    for i in 0..arr_len {
        println!("Index {}: {}", i, arr[i]);
    }

    // Search
    println!("'arr' contains 'dates': {}", arr.contains(&"dates"));
    println!("'arr' contains 'pears': {}", arr.contains(&"pears"));
    // Finds the first item only.
    println!("'dates' are at index {:?}",
             arr.iter().position(|val| *val == "dates"));
    println!("'horses' are at index {:?}",
             arr.iter().position(|val| *val == "horses"));

    // Insert (actually Replace)
    arr[1] = "pears";
    println!("'arr' after replace {:?}", arr);

    // Delete
}

fn foundations_vector() {
    let mut vec = vec!["apples", "bananas", "dates"];

    println!("\n*** Vector");
    println!("The size of 'vec' is {}", vec.len());
    println!("'vec' occupies {} bytes", size_of_val(&vec));

    // Read
    vec.iter().enumerate().for_each(|(i, val)| println!("Index {}: {}", i, val));

    // Search
    println!("'vec' contains 'dates': {}", vec.contains(&"dates"));
    println!("'vec' contains 'pears': {}", vec.contains(&"pears"));
    // Finds all items.
    vec
        .iter()
        .enumerate()
        .filter(|&(_i, val)| *val == "dates")
        .for_each(|(i, _val)| println!("Found 'dates' at index {i}"));

    // Insert
    vec.insert(1, "horses");
    vec.push("pears");
    println!("'vec' after inserts is {:?}", vec);

    // Delete
    vec.remove(1);
    vec.pop();
    println!("'vec' after removals is {:?}", vec);
}


fn foundations_set() {
    let set = HashSet::from(["apples", "bananas", "dates"]);

    println!("\n*** HashSet (unordered)");
    println!("The size of 'set' is {}", set.len());
    println!("'set' occupies {} bytes", size_of_val(&set));

    // Read
    for val in set {
        println!("{val}");
    }
}
