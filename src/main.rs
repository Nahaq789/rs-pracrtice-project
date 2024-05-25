mod cast_string_into_i32;

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("error");

    let input = input.trim();
    let input: i32 = input.parse().unwrap();

    let a: u32 = input as u32;

    println!("{}", a + power(a, 2) + power(a, 3));
}

fn power(a: u32, x: u32) -> u32 {
    a.pow(x)
}