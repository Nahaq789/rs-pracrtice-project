fn cast_string_into_i32() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("TODO: panic message");

    let input = input.trim();
    let mut input: i32 = input.parse().unwrap();

    println!("{}", input);
}