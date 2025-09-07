static mut gat:[u64;1001]=[0;1001];
static mut x:usize=0;
fn main(){
    unsafe{calcul(2);
    gat.sort();
    for i in gat{print!("{i}, ")};}
}
fn calcul(num:u64){
    unsafe{if gat.contains(&num) || x>=1000 {return;}
    x=x+1;
    gat[x]=num;
    if (num-1)%3==0{ calcul((num-1)/3);}
    if num%2==0 {calcul(num<<1);}}
}