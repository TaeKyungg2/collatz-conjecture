use rayon::prelude::*;
use std::io::{self, Write};

pub fn para(goal_num: u128) {
    (3..goal_num)
        .into_par_iter()
        .filter(|x| x % 2 == 1)
        .for_each(|i| {
            crate::calculation(i);
            if i % (goal_num / 16 - 1) == 0 {
                print!("{i} ");
            }
            io::stdout().flush().unwrap();
        });
}
pub fn nopara(goal_num: u128){
    print!("zzz");
}
pub fn calculation(i: u128) {
    let mut n = i;
    while n >= i {
        if n % 2 == 0 {
            n = n >> 1;
        } else {
            n = (n << 1) + n + 1;
        }
    }
}
