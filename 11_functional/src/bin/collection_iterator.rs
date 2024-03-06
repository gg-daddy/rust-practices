use std::collections::HashMap;

fn main() {
    let mut vec1 = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    // vec1.iter().for_each(|x| println!("{}", x));
    // vec1.iter_mut().for_each(|x| println!("{}", x));
    // vec1.into_iter().for_each(|x| println!("{}", x));

    for x in vec1.iter() {
        println!("{}", x);
    }
    for x in vec1.iter_mut() {
        println!("{}", x);
    }
    for x in vec1.into_iter() {
        println!("{}", x);
    }

    let mut h = HashMap::new();
    h.insert("k1", 12);
    h.insert("k2", 13);
    h.insert("k3", 14);

    // h.iter().for_each(|(k, v)| println!("{}:{}", k, v));
    // h.iter_mut().for_each(|(k, v)| println!("{}:{}", k, v));
    // h.into_iter().for_each(|(k, v)| println!("{}:{}", k, v));

    for (k, v) in h.iter() {
        println!("{}:{}", k, v);
    }
    for (k, v) in h.iter_mut() {
        println!("{}:{}", k, v);
    }
    for (k, v) in h.into_iter() {
        println!("{}:{}", k, v);
    }
}
