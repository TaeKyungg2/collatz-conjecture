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

fn main(){
    let mut i:i32=1;
    while i<100{
        let first=i;
        let mut n=i;
        while n>=first{
        if n%2==0{
            n=n>>1;
        }
        else{
            n=(n<<1)+n+1;
        }
        i+=1;
    }
    }
    println!("end");
}