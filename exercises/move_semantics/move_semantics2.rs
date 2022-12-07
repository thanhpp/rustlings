// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let mut vec0: Vec<i32> = Vec::new();

    // // solution 1: another vet
    // let vec2 = vec0.clone();

    // // solution 2: borrow its argument instead of taking ownership of it
    // let mut vec1 = fill_vec(&vec0);

    // solution 3: use mutable reference
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
