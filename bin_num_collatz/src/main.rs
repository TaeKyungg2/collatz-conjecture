use std::io;
fn main(){
    let mut select=String::new();
    let _ = io::stdin().read_line(&mut select);
    let value=6;
    let base_n:u64=0b10111;
    if select=="1\n"{
        println!("num_be1");
        num_be1_counts(base_n);
    }
    else{
        println!("Ncount");
        n_count_not_find(value, base_n);
    }
}
fn num_be1_counts(base_n:u64){
    let mut value;
    let length=64-base_n.leading_zeros();
    let mut appear_count=[0;400];
    let mut caled_num:u64;
    for i in 1..100{
        caled_num=base_n+(i<<length);
        value=count(caled_num);
        appear_count[value]+=1;
    }
    for i in 0..400{
        if appear_count[i]!=0{
            println!("{} is appear {} count.",i,appear_count[i]);
        }
    }
}

fn n_count_not_find(value:usize,base_n:u64){
    let mut caled_num:u64;
    let length=64-base_n.leading_zeros();
    let mut flag:bool=true;
    for i in 1..100000{
        caled_num=base_n+(i<<length);
        if count(caled_num) !=value{
            println!("{:16b}is not 13",caled_num);
            flag=false;
        }
    }
    if flag{
        println!("all pass");
    }
    
}
fn count(mut n:u64)->usize{
    let first:u64=n;
    let mut count:usize=0;
    while n>=first{
        count=count+1;
        if n%2==0{
            n=n>>1;
        }
        else{
            n=(n<<1)+n+1;
        }
    }
    count
}