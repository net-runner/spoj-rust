use std::cmp;
use std::io;

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line!");
    buffer
}
fn get_uint_from_input() -> u32 {
    get_input().trim().parse::<u32>().unwrap()
}
fn get_input_to_vec_uint() -> Vec<u32> {
    let vec: Vec<u32> = get_input()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("String parse error"))
        .collect();
    vec
}
fn main() {
    let tests = get_uint_from_input();
    let mut zex: u32;
    let mut zey: u32;
    let mut ex: [u32; 40001] = [0; 40001];
    let mut ey: [u32; 40001] = [0; 40001];
    for _ in 0..tests {
        let first_params = get_input_to_vec_uint();
        let w = first_params[0];
        let h = first_params[1];
        let towers = first_params[2];

        if towers == 0 {
            println!("{}", w * h);
            continue;
        }

        let mut penalty = 0;

        if towers == 1 {
            let tower_coords = get_input_to_vec_uint();
            zex = tower_coords[0] - 1;
            zey = tower_coords[1] - 1;
            penalty = cmp::max(penalty, zex * zey);
            zey = h - tower_coords[1];
            penalty = cmp::max(penalty, zex * zey);
            zex = w - tower_coords[0];
            penalty = cmp::max(penalty, zex * zey);
            zey = tower_coords[1] - 1;
            penalty = cmp::max(penalty, zex * zey);
            println!("{}", penalty);
            continue;
        }

        for i in 1..(towers + 1) {
            let tower_coords = get_input_to_vec_uint();
            ex[i as usize] = tower_coords[0];
            ey[i as usize] = tower_coords[1];
        }
        ex[0] = 0;
        ex[(towers + 1) as usize] = w + 1;
        ey[0] = 0;
        ey[(towers + 1) as usize] = h + 1;
        &ex[0..(towers + 1) as usize].sort_unstable();
        &ey[0..(towers + 1) as usize].sort_unstable();

        zex = 0;
        zey = 0;
        for index in 1..(towers + 2) {
            zex = cmp::max(zex, ex[index as usize] - ex[(index - 1) as usize] - 1);
            zey = cmp::max(zey, ey[index as usize] - ey[(index - 1) as usize] - 1);
        }
        println!("{}", zex * zey);
    }
}
