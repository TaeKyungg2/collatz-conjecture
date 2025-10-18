use std::io;

fn main(){
    let mut select=String::new();
    let _ = io::stdin().read_line(&mut select);
    let value=6;
    let base_n:u128=0b10111;

    if select=="1\n"{
        println!("num_besmall");
        num_besmall_counts(base_n);
    }
    else{
        println!("Ncount");
        n_count_not_find(value, base_n);
    }
}
fn num_besmall_counts(base_n:u128){
    let mut value;
    let length=128-base_n.leading_zeros();
    let mut appear_count=[0;400];
    let mut caled_num:u128;
    for i in 1..10000{
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

fn n_count_not_find(value:usize,base_n:u128){
    let mut caled_num:u128;
    let length=128-base_n.leading_zeros();
    let mut flag:bool=true;
    for i in 1..100{
        caled_num=base_n+(i<<length);
        if count(caled_num) ==value{
            println!("{:16b}is{}",caled_num,value);
            flag=false;
        }
    }
    if flag{
        println!("all pass");
    }
    
}
fn count(mut n:u128)->usize{
    let first:u128=n;
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