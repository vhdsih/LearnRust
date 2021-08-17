use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("xiaozhang"), 1);
    scores.insert(String::from("xiaowang"), 2);
    scores.insert(String::from("xiaoliu"), 3);
    scores.insert(String::from("xiaoliu"), 3222);

    let name = String::from("xiaoliu");
    let val = scores.get(&name);
    println!("get xiaoliu {:#?}", val);
    scores.entry(name).or_insert(999);


    let name = String::from("xiaoliuuuuu");
    scores.entry(name).or_insert(999);
    let name = String::from("xiaoliuuuuu");
    let val = scores.get(&name);
    println!("get xiaoliu {:#?}", val);


    for (key, val) in &scores {
        println!("key {} value {}", key, val);
    }

    let keys = vec![String::from("v"), String::from("p"), String::from("p"), String::from("p")];
    let vals = vec![1, 2, 3, 4];
    let dict : HashMap<_, _> = keys.into_iter().zip(vals.into_iter()).collect();

    for (key, val) in &dict {
        println!("key {} value {}", key, val);
    }

    let text = "this is a text in this file";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:?}", map);

}
