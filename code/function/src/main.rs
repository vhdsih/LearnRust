fn test() {
    println!("This is test fn");
}

fn test_args(x: i32, y: u32) {
    println!("The arg is x:{}, y:{}", x, y);
}

fn return_val(x : i32) -> i32 {
    /*
    if (x) {
        return 128;
    } else {
        return 58;
    }
    */
    99 + x
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");
    test();
    test_args(22, 33);
    let ret = return_val(12);
    println!("return val is {}", ret);
    let one = plus_one(129);
    println!("add 1 is {}", one);
}
