
use std::fs::{OpenOptions, File};
use std::io::Write;

fn main(){
    let mut file=OpenOptions::new()
        .create(true)
        .append(true)
        .open("nums.txt")
        .unwrap();
    let mut cash:Vec<String>=Vec::new();
    for i in 2..10000{
        let road=cal(i,&mut cash);
        writeln!(file, "{}",road).unwrap();
    }
}

fn cal(n:u128,cash:&mut Vec<String>)->String{
    let mut count:usize=0;
    let mut num=n;
    let mut road=String::new();
    while num!=1{
        count=count+1;
        if num%2==0{
            num=num>>1;
        }
        else{
            num=(num<<1)+num+1;
        }
    }
    road
}