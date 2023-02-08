use std::collections::{BTreeSet, HashSet};
use std::mem::size_of_val;

pub fn review_array() {
    let mut arr = ["apples", "bananas", "dates", "dates"];

    println!("\nArray: {arr:?}");
    println!("The size of 'arr' is {}", arr.len());
    println!("'arr' occupies {} bytes", size_of_val(&arr));

    //// Read
    let i = 1;
    if let Some(val) = arr.get(i) {
        println!("Read value at index {i}: {val:?}");
    }

    //// Search
    // Linear search.
    let lookup = "dates";
    for (i, val) in arr.iter().enumerate() {
        if val.eq(&lookup) {
            println!("Found {val:?} at index {i}");
        }
    }

    //// Insert
    //// Delete
    // Kinda 🫣. Array is a fixed-size type in Rust.
    // Since we're playing around with the basics,
    // let's replace a value at an index
    // and mention a dynamically-sized slice type.
    // Also, we review a vector type (growable array)
    // in the next function.
    arr[1] = "pears";
    println!("After replace: {arr:?}");

    let s = &arr[1..];
    println!("Slice without 'apples': {s:?}");
}

pub fn review_vector() {
    let mut vec = vec!["apples", "bananas", "dates", "dates"];

    println!("\nVector: {vec:?}");
    println!("The size of 'vec' is {}", vec.len());
    println!("'vec' occupies {} bytes", size_of_val(&vec));

    //// Read
    let i = 1;
    if let Some(val) = vec.get(i) {
        println!("Read value at index {i}: {val:?}");
    }

    //// Search
    // Linear search. Find all occurrences.
    let lookup = "dates";
    for (i, val) in vec.iter().enumerate() {
        if val.eq(&lookup) {
            println!("Found {val:?} at index {i}");
        }
    }

    //// Insert
    vec.insert(1, "horses");
    // Append to the back of collection.
    vec.push("pears");
    println!("After inserts: {vec:?}");

    //// Delete
    vec.remove(1);
    // Remove the last element.
    vec.pop();
    println!("After removals: {vec:?}");
}

pub fn review_hashset() {
    let mut set = HashSet::from(["apples", "bananas", "dates", "dates"]);

    println!("\nHashSet: {set:?}");
    println!("The size of 'set' is {}", set.len());
    println!("'set' occupies {} bytes", size_of_val(&set));

    //// Read
    if let Some(val) = set.get(&"bananas") {
        println!("Got reference to {val:?}");
    }

    //// Search
    let lookup = "dates";
    println!("'set' contains {:?}: {}", lookup, set.contains(lookup));

    //// Insert
    set.insert("horses");
    set.insert("dates");
    println!("After inserts: {set:?}");

    //// Delete
    set.remove("horses");
    println!("After removal: {set:?}");
}

pub fn review_btreeset() {
    let mut set = BTreeSet::from(["apples", "dates", "dates", "bananas"]);

    println!("\nBTreeSet: {set:?}");
    println!("The size of 'set' is {}", set.len());
    println!("'set' occupies {} bytes", size_of_val(&set));

    //// Read
    if let Some(val) = set.get(&"bananas") {
        println!("Got reference to {val:?}");
    }

    //// Search
    let lookup = "dates";
    println!("'set' contains {:?}: {}", lookup, set.contains(lookup));

    //// Insert
    set.insert("horses");
    set.insert("dates");
    println!("After inserts: {set:?}");

    //// Delete
    set.remove("horses");
    println!("After removal: {set:?}");
}
