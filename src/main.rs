use proconio::input;

fn main() {
    input! {
        mut a500: i32, mut b100: i32, mut c50: i32, x: i32
    }
    let mut count = 0;
    for a in 0..=a500 {
        for b in 0..=b100 {
            for c in 0..=c50 {
                let mut total = 500 * a + 100 * b + 50 * c;
                if total == x {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count)
}