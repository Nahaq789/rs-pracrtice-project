// use proconio::input;
//
// mod Shift_Only;
//
// fn main() {
//     input! {
//         N: i32,
//         mut arr: [i32; N]
//     }
//     let mut count = 0;
//     let mut flg: bool = true;
//     loop {
//         for i in 0..arr.iter().count() {
//             if arr[i] % 2 != 0 {
//                 flg = false
//             }
//         }
//         if flg == false { break; }
//         for i in 0..arr.iter().count() {
//             arr[i] /= 2;
//         }
//         count += 1;
//     }
//     println!("{}", count)
// }
