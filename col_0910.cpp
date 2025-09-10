#include<iostream>
#include<set>
#include<cmath>
#include <cstdlib>
#include <fstream>
#include <cstring>
#include <fstream>
using namespace std;
void cal(unsigned long long int n);
bool is_be_1[100000]={};
unsigned long long int road[10000]={};
fstream f;
unsigned int x=0;
int main(){
    f.open("../col_dp_data.txt", ios::in | ios::out);
    for(unsigned long long int i=2;i<=10000;i++){
        cal(i);
        x=0;
        memset(road, 0, sizeof(road));
    }
    f.close();
    return 0;
}
unsigned long long int big_int=100000000000000;
void cal(unsigned long long int n){
    x+=1;
    road[x]=n;
    if(n==1){//outFile << "Be 1 in count : "<<x << endl;
        for(int i =1;i<=10000;i++){
            if(road[i]==0){
            f<<endl;
            return;
            }
            f << road[i] << ", ";
        }
        f << endl;
        return;
    }
    // if(n<100000 && is_be_1[n])
    // {
    //     f.read()
    //     file << 
    //     file << endl;
    //     return;
    // }
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