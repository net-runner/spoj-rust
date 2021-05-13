use std::io;
fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line!");
    buffer
}
fn get_input_to_vec_uint() -> Vec<u32> {
    let vec: Vec<u32> = get_input()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("String parse error"))
        .collect();
    vec
}
fn arthprog(vectorum: &mut Vec<u32>, a: u32, b: u32, direction: i32, n: u32, current: i32) {
    let mortituro = a as i32 + (direction * b as i32 * current) - 1;
    if mortituro >= 0 && mortituro < n as i32 {
        vectorum[mortituro as usize] += 1;
        arthprog(vectorum, a, b, direction, n, current + 1);
    }
}

fn main() {
    let tests = get_input().trim().parse::<u32>().unwrap();
    for t in 0..tests {
        let first_params = get_input_to_vec_uint();
        let n = first_params[0];
        let q = first_params[1];

        let mut vectorum: Vec<u32> = vec![Default::default(); n as usize];
        for _ in 0..q {
            let query_params = get_input_to_vec_uint();
            let a = query_params[0];
            let b = query_params[1];
            if a as usize <= n as usize && a >= 1 && b >= 1 && b as usize <= n as usize {
                arthprog(&mut vectorum, a, b, -1, n, 0);
                arthprog(&mut vectorum, a, b, 1, n, 1);
            }
        }
        print!("Case {}:", t + 1);
        for int in vectorum {
            print!(" {}", int);
        }
        println!("");
    }
}
