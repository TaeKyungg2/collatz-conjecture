// %%writefile coll_nums_100m.cu
#include <cstdlib>
#include <fstream>
#include <cstring>
#include <fstream>
using namespace std;

__device__ void cal(unsigned long long int n,unsigned long long int road[1000],unsigned int x);
__device__ __managed__ unsigned long long int all_road[100000001][1000]={};
fstream f;

__global__ void go_cal(){
    for(unsigned int i=2;i<=100000000;i++){
        unsigned long long int road[1000];
        unsigned int x=0;
        cal(i,road,x);
    }
}
int main(){
    f.open("col_dataset_100m.txt", ios::in | ios::out);
    cudaDeviceSetLimit(cudaLimitStackSize, 256 * 1024);
    go_cal<<<256,256>>>();
    for(int i=2;i<100000001;i++){
        for(int j=1;j<1000;j++){
            f << all_road[i][j] << ",";
        }
        f << endl;
    }
    f.close();
    cudaDeviceSynchronize();
    return 0;
}

__device__ unsigned long long int big_int=18446744073709551614;

__device__ void cal(unsigned long long int n,unsigned long long int road[1000],unsigned int x){
    x+=1;
    if(x>=1000){printf("%llu over 1000 count.",n);}
    road[x]=n;
    if(n==1){
        for(int i=1;i<1000;i++){
            if(road[i]==0){ return;}
            all_road[road[1]][i]=road[i];
        }
        return;
    }
    if(n%2==0){
        cal(n>>1,road,x);
    }
    else{
        if(n>=big_int){
            printf("start_num: %llu over",road[1]);
            return;
        }
        cal((n<<1)+n+1,road,x);
    }
}