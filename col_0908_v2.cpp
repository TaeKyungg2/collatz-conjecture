#include<iostream>
#include<set>
#include<cmath>
#include <cstdlib>
using namespace std;
void cal(unsigned long long int n);
bool is_be_1[100000]={};
unsigned int x=0;
int main(){
    unsigned long int x=0;
    unsigned long long int n=19990012341299;
    cal(n);
    return 0;
}
unsigned long long int big_int=100000;
void cal(unsigned long long int n){
    x+=1;
    if(n==1){
        cout << "Be 1 in count : "<<x;
        exit(0);
    }
    if(n%2==0){
        cal(n>>1);
    }
    else{
        if(n>=big_int){
            cout << "count: "<<x<<" overnum is"<<n;
            exit(0);
        }
        cal((n<<1)+n+1);
    }
}