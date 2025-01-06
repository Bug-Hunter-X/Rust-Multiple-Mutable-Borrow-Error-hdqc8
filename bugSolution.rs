fn main() {
    let mut x = 5;
    { // Using a separate scope limits the life of the mutable borrow
        let y = &mut x;
        *y += 1; 
    }
    let z = &mut x; 
    *z += 1; // No panic because of the separate scope in the example above
    println!("x = {}", x);
}
//Alternative solution using clone
fn main() {
    let mut x = 5;
    let mut y = x;
    let mut z = x;
    y += 1; 
    z += 1; 
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
} 