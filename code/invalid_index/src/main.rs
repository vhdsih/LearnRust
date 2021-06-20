use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("input index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Faile to read line");

    let index : usize = index.trim().parse().expect("Index not a number"); 

    let element = a[index];

    println!("value is {}, index is {}", element, index);
}