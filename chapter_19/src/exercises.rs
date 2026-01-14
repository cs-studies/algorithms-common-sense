pub fn reverse_1<T: Clone>(v: &[T]) -> Vec<T> {
    let mut new_v = Vec::with_capacity(v.len());
    for i in (0..v.len()).rev() {
        new_v.push(v[i].clone());
    }
    new_v
}

pub fn reverse_2<T>(mut v: Vec<T>) -> Vec<T> {
    let len = v.len();
    for i in 0..len/2 {
        v.swap(i, len - 1 - i);
    }
    v
}

