use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        N: i32, L: i32, mut R: i32
    }
    let mut a: Vec<i32> = vec![];
    let mut b: Vec<i32> = vec![];

    let mut scores: HashMap<i32, i32> = HashMap::new();

    for i in 1..N + 1 {
        a.push(i)
    }

    a.sort();
    for i in a.iter() {
        scores.insert(*i, *i);
    }


    for i in L..=R {
        b.push(i);
    }

    b.reverse();

    for i in b.iter() {
        let index = a.iter().position(|x| x == i).unwrap();
        a.remove(index);
    }
    for i in b.iter() {
        a.push(*i);
    }

    println!("{:?}", scores);
}