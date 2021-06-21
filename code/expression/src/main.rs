fn main() {
    let x = 1;
    let y = {
        let x = 10;
        x + 1
    };
    println!("x out is {}", x);
    println!("y is {}", y);
}
