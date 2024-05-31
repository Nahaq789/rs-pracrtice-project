// use proconio::input;
//
// fn main() {
//     input! {
//         N: usize,
//         T: f64,
//         A: f64,
//         H: [f64; N]
//     }
//     let mut avg_temp = f64::MAX;
//     let mut ans = usize::MAX;
//     let mut macnt = f64::MAX;
//
//     for x in 0..H.len() {
//         let temp = T - (H[x] * 0.006);
//         let abs = (temp - A as f64).abs();
//         if (abs < macnt) {
//             macnt = abs;
//             ans = x;
//         }
//     }
//     println!("{}", ans + 1)
// }
