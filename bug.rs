fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x
    *y += 1; // Modifies x through y
    *z += 1; // This will panic at runtime due to multiple mutable references to x
    println!("x = {}", x);
}