use rayon::prelude::*; 
use std::io::{self, Write};
use std::time::Instant;
fn main(){
    let start = Instant::now();
    let goal_num=1 << 25;
    (3..goal_num).into_par_iter()
    .filter(|x| x % 2 == 1)
        .for_each(|i| {
        calculation(i);
        if i%(goal_num/16-1)==0{print!("{i} ");}
        io::stdout().flush().unwrap();
    });
    println!("end {goal_num} numbers");
    let elapsed = start.elapsed();
    println!("걸린 시간: {:?}", elapsed);
}
fn calculation(i:u128){
    let mut n=i;
    while n>=i{
        if n%2==0{n=n>>1;}
        else{n=(n<<1)+n+1;}
    }
}
