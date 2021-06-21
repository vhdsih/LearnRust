fn main() {
    let x = 7;
    if x > 5 {
        println!("x > 5");
    } else {
        println!("x <= 5");
    }

    let y = true;
    if y {
        println!("y is not 0");
    }

    let z = if y { 199 } else { 299 };

    println!("z is {}", z);

    let mut counter = 1;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
