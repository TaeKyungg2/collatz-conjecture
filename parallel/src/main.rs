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
//use rayon::prelude::*; 
fn main(){
    let two:u128=2;
    let goal_num=two.pow(30);
    let mut i:u128=3;
    while i<10{
        calculation(i);
        i+=2;
    }
    println!("end {goal_num} numbers");
}

fn calculation(i:u128){
    let mut n=i;
    while n>=i{
        if n%2==0{
            n=n>>1;
        }
        else{
            n=(n<<1)+n+1;
        }
    }
}