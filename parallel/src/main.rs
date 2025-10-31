// use std::io::{self, Write};
use std::time::Instant;
fn main() {
    let goal_num = 1 << 20;
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("help");

    let start = Instant::now();

    match mode {
        "para" => parallel::para(goal_num),
        "nopara" => parallel::nopara(goal_num),
        _ => eprintln!("사용법 : cargo run -- [para|nopara]"),
    }
    println!("end {goal_num} numbers");
    let elapsed = start.elapsed();
    println!("걸린 시간: {:?}", elapsed);
}
