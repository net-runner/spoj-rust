//AKA find factorial trailing zeros
use std::io;
fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line!");
    buffer
}
fn get_uint_from_input() -> u64 {
    get_input().trim().parse::<u64>().unwrap()
}
fn find_zeroes(n: u64) {
    let mut count: u64 = 0;
    let mut i: u64 = 5;
    while n / i > 0 {
        count += n / i;
        i *= 5;
    }
    println!("{}", count);
}
fn main() {
    let tests = get_uint_from_input();

    for _ in 0..tests {
        let numbr = get_uint_from_input();
        find_zeroes(numbr)
    }
}
