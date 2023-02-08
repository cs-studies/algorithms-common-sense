pub fn evens_v1() {
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
    println!("evens_v1: {count} 'while' and {count_ifs} 'if' calls.");
}

pub fn evens_v2() {
    let mut num = 2;
    let mut count = 0;

    while num <= 100 {
        // println!("{num} is even.");
        num += 2;
        count += 1;
    }
    println!("evens_v2: {count} 'while' calls.");
}

//// Rust Extras

pub fn evens_step_by() {
    let mut count = 0;

    for _num in (2..=100).step_by(2) {
        // println!("{_num} is even");
        count += 1;
    }
    println!("evens_step_by: {count} 'for' loops.");
}

pub fn evens_filter() {
    let mut count = 0;

    (2..=100).filter(|n| n % 2 == 0).for_each(|_n| {
        // println!("{_n}");
        count += 1
    });

    println!("evens_filter: {count} 'for_each' loops.");
}
