use rayon::prelude::*;
// use std::io::{self, Write};

pub fn para(goal_num: u128) {
    println!("parallel start");
    (3..goal_num)
        .into_par_iter()
        .filter(|x| x % 2 == 1)
        .for_each(|i| {
            crate::calculation(i);
        });
}
pub fn nopara(goal_num: u128){
    println!("not parallel start");
    for n in (3..goal_num).step_by(2){
        crate::calculation(n);
    }
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
