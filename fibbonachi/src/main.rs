fn main() {
    let mut a: i128 = 1;
    let mut b: i128 = 1;
    for _ in 1..49{
        let new = a + b;
        a = b;
        b = new;
    }
    println!("{b}")
}
