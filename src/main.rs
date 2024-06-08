use proconio::input;

fn main() {
    input! {
        n: i64
    }
    let mut s = String::new();
    let mut result: i64 = 0;
    let mut count = 0;
    for i in 0..n {
        //s += &*n.to_string();
        count += n % 998244353 * 11111;
    }
    //result = (s.parse::<i64>().unwrap()) % 998244353;
    println!("{}", count)
}