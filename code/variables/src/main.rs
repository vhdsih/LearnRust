const A : u32 = 123;
fn main() {
    let x = 5;
    println!("x is {}, a is {}", x, A);

    let x = x + 1;
    let x = x + 1;
    println!("x is {}, a is {}", x, A);

    let y = 11_22;
    println!("y is {}", y);

    let z: f64 = 8.22_00_11;
    println!("z is {}", z);

    let mut aaa = 1;
    println!("aaa is {}", aaa);
    aaa = x * y;
    println!("aaa is {}, bbb is {}", aaa, 5 / 2);

    let p = true;
    let q = false;
    println!("p is {}, q is {}", p, q);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c is {}, z is {}, heart_eyed_cat is {}", c, z, heart_eyed_cat);
}

