const POINT:u128=2_u128.pow(64);
const POINT_LAST:u128=2_u128.pow(126);
fn main(){
    let mut flag = String::from("all pass");
    for i in POINT..POINT+10000{
        if cal(i) {
            print!("{}g ",i);//good
        }
        else{
            print!("{}b ",i);//bad
            flag=format!("false at {}",i)
        }
    }
    print!("{}",flag);
}
fn cal(n:u128)->bool{
    if n<POINT{
        true
    }
    else if n%2==0 {
        cal(n>>1)
    }
    else if n<POINT_LAST {
        cal((n<<1)+n+1)
    }
    else{
        print!("over_2^126");
        false
    }
}