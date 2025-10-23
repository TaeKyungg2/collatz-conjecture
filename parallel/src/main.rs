// use std::thread;
// use std::time::Duration;

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(50));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(100));
//     }
// }
//use bitvec::prelude::*;
    // 
    // 
    // println!("{b}");
    // let mut bits = bitvec![0; 1000000000000];
    // let nums = vec![1, 2, 3, 4, 5];
    // let squares: Vec<_> = nums.par_iter().map(|x| x * x).collect();
    // println!("{:?}",squares);
    // fn main1(){
    //     let start = Instant::now();
    //     let goal_num=1<<5;
    //     let mut i:u128=3;
    //     while i<goal_num{
    //         calculation(i);
    //         i+=2;
    //         if i%(goal_num/16-1)==0{print!("{i} ");}
    //         io::stdout().flush().unwrap();
    //     }
    //     println!("end {goal_num} numbers");
    //     let elapsed = start.elapsed();
    //     println!("걸린 시간: {:?}", elapsed);
    // }
use rayon::prelude::*; 
use std::io::{self, Write};
use std::time::Instant;
fn main(){
    let start = Instant::now();
    let goal_num=1 << 5;
    (3..goal_num).into_par_iter().filter(|x| x % 2 == 1).for_each(|i| {
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
        if n%2==0{
            n=n>>1;
        }
        else{
            n=n<<1+n+1;
        }
    }
}
