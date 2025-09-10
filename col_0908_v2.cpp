#include<iostream>
#include<set>
#include<cmath>
#include <cstdlib>
#include <fstream>
#include <cstring>
using namespace std;
void cal(unsigned long long int n);
bool is_be_1[100000]={};
unsigned long long int road[10000]={};
ofstream outFile("col.txt");
unsigned int x=0;
int main(){
    for(unsigned long long int i=2;i<=10000;i++){
        cal(i);
        x=0;
        memset(road, 0, sizeof(road));
    }
    outFile.close();
    return 0;
}
unsigned long long int big_int=100000000000000;
void cal(unsigned long long int n){
    x+=1;
    road[x]=n;
    if(n==1){
        //outFile << "Be 1 in count : "<<x << endl;
        for(int i =1;i<=10000;i++){
            if(road[i]==0){
            outFile<<endl;
            return;
            }
            outFile << road[i] << ", ";
        }
        outFile << endl;
        return;
    }
    if(n<100000 && is_be_1[n])
    {
        outFile << endl;
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