enum Coin {
    YiJiao,
    WuJiao,
    YiYuan,
    BuCunZai(u32),
}

fn match_coin(coin: Coin) -> u32 {
    match coin {
        Coin::YiJiao => 1,
        Coin::WuJiao => 5,
        Coin::YiYuan => 10,
        Coin::BuCunZai(value) => value,
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_some_value(val: u8) {
    match val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn main() {
    let coin1 = Coin::WuJiao;
    let coin2 = Coin::YiJiao;
    let coin3 = Coin::YiYuan;
    let coin4 = Coin::BuCunZai(100);
    println!(" value={}", match_coin(coin1));
    println!(" value={}", match_coin(coin2));
    println!(" value={}", match_coin(coin3));
    println!(" value={}", match_coin(coin4));

    let x = 1;
    let y = plus_one(Some(x));
    let z = plus_one(None);
    println!("{:?} {:?} {:?}", x, y, z);

    match_some_value(1);

    if let 1 = x {
        println!("x = 1");
    }
}
