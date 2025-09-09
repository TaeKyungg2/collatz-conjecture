
fn main(){
    let mut gat:[u64;1001]=[0;1001];
    let mut x:usize=0;
    calcul(2);
    gat.sort();
    for i in gat{print!("{i}, ")};

    fn calcul(num:u64){
        if gat.contains(&num) || x>=1000 {return;}
        x=x+1;
        gat[x]=num;
        if (num-1)%3==0{ calcul((num-1)/3);}
        if num%2==0 {calcul(num<<1);}
    }
}
