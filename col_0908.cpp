#include<iostream>
#include<set>
#include<queue>
using namespace std;
set<int> gat;
void calcul(int num);
queue<int> qu;
int main(){
    qu.push(2);
    for(int i=0;i<=1000000;i++){
        if(qu.empty()){
            cout << i <<"번째 FOR 에서 큐가 비었다."<< "\n";
            break;
        }
        calcul(qu.front());
        qu.pop();
    }
    for(int i:gat){
        if(i>10000){continue;}
        cout << i << ", ";
    }
}
void calcul(int num){
    if(gat.find(num)!=gat.end()|| num<=1||num>=1000000){return;}
    gat.insert(num);
    if((num-1)%3==0){ qu.push((num-1)/3);}
    qu.push(num<<1);
}