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
fn main(){
    let two:u128=2;
    let goal_num=two.pow(30);
    let mut i:u128=1;
    while i<goal_num{
        let mut n=i;
        while n>=i{
            if n%2==0{
                n=n>>1;
            }
            else{
                n=(n<<1)+n+1;
            }
            i+=2;
        }
    }
    println!("end{goal_num}numbers");
}