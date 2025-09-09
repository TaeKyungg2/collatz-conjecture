#include<iostream>
#include<set>
#include<queue>
using namespace std;
set<int> gat;
void calcul(int num);
queue<int> qu;
int main(){
    qu.push(2);
    int[] load=[10000];
    for(int i=0;i<=1000000;i++){
        if(qu.empty()){
            cout << i <<"번째 FOR 에서 큐가 비었다."<< "\n";
            break;
        }
        calcul(qu.front(),load);
        qu.pop();
    }
    for(int i:gat){
        if(i>10000){continue;}
        cout << i << ", "; // 트리에 포함된 숫자를 10000보다 작은 숫자만 출력한다.
    }
    
}
void calcul(int num,int[] load){
    if(gat.find(num)!=gat.end()|| num<=1||num>=1000000){return;}
    gat.insert(num);
    if((num-1)%3==0){ qu.push((num-1)/3);}
    qu.push(num<<1);
}