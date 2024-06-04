use proconio::input;

fn main() {
    input! {

        n: i32, a: i32, b: i32
    }

    let mut res = 0;
    for n in 0..=n {
        let mut n_string = n.to_string();
        let mut n_iter = n_string.chars().map(|n| n.to_digit(10).unwrap());
        let n_sum: i32 = n_iter.fold(0, |sum, n| sum + n as i32);

        if n_sum >= a && n_sum <= b {
            res += n;
        }
    }
    println!("{}", res);
}