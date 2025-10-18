//const MAX:u128=340282366920938463463374607431768211455;
fn main(){
    let goal_num=100_000_000;
    let mut number:u128=1;
    while number<goal_num{
        number+=2;
        let mut one=0;
        let mut zero=0;
        let mut big_zero=0;
        let mut big_one=0;
        let mut iszero=false;
        let length=128-number.leading_zeros();
        for j in 0..length{
            if number&1<<j==0{
                one=0;
                if iszero{
                    zero+=1;
                    if zero>big_zero{
                        big_zero=zero;
                    }
                }
                iszero=true;
            }
            else{
                zero=0;
                if !iszero{
                    one+=1;
                    if one>big_one{
                        big_one=one;
                    }
                }
                iszero=false;
            }
        } 
        //println!("number{:b} zero{} one{}",number,big_zero,big_one);
        if big_zero>big_one{
            continue;
        }
        count(number);
    }
    println!("{}",goal_num);
}
fn count(mut n:u128)->bool{
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
    true
}