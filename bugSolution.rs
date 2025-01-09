fn main() {
    let mut x = 5;
    { // Use a block to limit the scope of y
        let y = &mut x;
        *y = 6;
    }
    let z = &mut x; //Now z is the only mutable reference
    *z = 7; 
    println!("x = {}", x);
}