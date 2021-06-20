fn main() {
    println!("Hello, world!");
    test();
    test_args(22);
}

fn test() {
    println!("This is test fn");
}

fn test_args(x: i32) {
    println!("The arg is {}", x);
}