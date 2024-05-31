// use proconio::input;
//
// fn main() {
//     input! {
//         N: i32,
//     }
//
//     let mut macnt = -1;
//     let mut ans = -1;
//     if N == 1 {
//         println!("{}", N);
//         return;
//     }
//     for i in 1..N + 1 {
//         let mut count = 0;
//         let mut y = i;
//         while y % 2 == 0 {
//             y /= 2;
//             count += 1;
//         }
//         if count > macnt {
//             macnt = count;
//             ans = i as i32;
//         }
//     }
//     println!("{}", ans)
// }
