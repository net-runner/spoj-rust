use std::io;
const MAXLEN: usize = 500;
fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line!");
    buffer
}
fn multiply(x: u64, bigi: &mut [u64; MAXLEN], mut bigi_size: usize) -> usize {
    let mut carry: u64 = 0;
    for n in 0..bigi_size {
        let temp = bigi[n] * x + carry;
        bigi[n] = temp % 10;
        carry = temp / 10;
    }
    while carry != 0 {
        bigi[bigi_size] = carry % 10;
        carry /= 10;
        bigi_size += 1;
    }
    return bigi_size;
}
fn main() {
    let x = get_input().trim().parse::<u32>().unwrap();

    for _ in 0..x {
        let numbr = get_input().trim().parse::<u64>().unwrap();
        let mut bigi: [u64; MAXLEN] = [0; MAXLEN];
        bigi[0] = 1;
        let mut bigi_size: usize = 1;
        for n in 2..numbr + 1 {
            bigi_size = multiply(n, &mut bigi, bigi_size);
        }
        for o in (0..bigi_size).rev() {
            print!("{}", bigi[o]);
        }
        println!("");
    }
}
