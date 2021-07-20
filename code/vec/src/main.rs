fn main() {
    let v: Vec<i32> = Vec::new();
    let m = vec![1,2,3];
    println!("vec is {:#?} {:#?}", v, m);

    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);
    v1.push(5);

    println!("v1 is {:#?}", v1);

    let a = vec![1, 2, 3, 4, 5];

    println!("The third elemet is {}", a[2]);
    println!("The 100 elemet is {:?}", a.get(100));
    // println!("The 100 elemet is {}", a[100]);
    //println!("The 100 elemet is {}", &a[100]);

    let third: &i32 = &a[2];
    println!("The third elemet is {}", third);

    match a.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let q = vec![1, 3, 5, 7, 9];

    for i in &v {
        println!("x is {}", i);
    }

    let mut qq = vec![1, 1, 1, 1];

    for i in &mut qq {
        *i += 100;
    }
    println!("qq is {:#?}", qq);

    let mut idx = &mut qq[1];
    *idx = 10000;

    qq.pop();
    println!("qq is {:#?}", qq);


}
