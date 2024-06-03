use proconio::input;

fn main() {
    input! {
        n: i32, mut a: [i32; n]
    }
    let mut flg = true;
    let mut count = 0;
    loop {
        for x in 0..a.len() {
            if a[x] % 2 != 0 {
                println!("{}", count);
                return;
            }
        }
        for b in 0..a.len() {
            if a[b] % 2 == 0 {
                a[b] /= 2;
            } else {
                println!("{}", count);
                return;
            }
        }
        count += 1;
    }
}