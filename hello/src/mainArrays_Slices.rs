fn main() {
    let arr = [0, 1, 2, 3];
    let slice = &arr[1 .. 3];
    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}


// Paste in the code in main.rs to make it work and unmark the regular code in main.rs