#include<iostream>
#include<set>
#include<cmath>
#include <cstdlib>
#include <fstream>
#include <cstring>
#include <fstream>
using namespace std;
void cal(unsigned long long int n);
unsigned long long int road[10000]={};
fstream f;

unsigned int x=0;
int main(){
    cal(1000000023);
    return 0;
}
unsigned long long int big_int=1844674407370955161;
void cal(unsigned long long int n){
    x+=1;
    road[x]=n;
    if(n==1){
        cout << "Be 1 in count : "<<x << endl;
        for(int i =1;i<=10000;i++){
            if(road[i]==0){
            return;
            }
            cout << road[i] << ", ";
        }
        cout << endl;
        return;
    }
    if(n%2==0){
        cal(n>>1);
    }
    else{
        if(n>=big_int){
            cout << "count: "<<x<<" overnum is"<<n;
            return;
        }
        cal((n<<1)+n+1);
    }
}