#include <cstdlib>
#include <fstream>
#include <cstring>
using namespace std;

__device__ void cal(unsigned long long int n,unsigned long long int road[1000],unsigned int x);
__device__ __managed__ unsigned long long int all_road[10000][1000]={};


__global__ void go_cal(){
    for(unsigned int i=2;i<=10000;i++){
        unsigned long long int road[1000]={};
        unsigned int x=0;
        cal(i,road,x);
    }
}
int main(){
    fstream f;
    f.open("../col_dataset_100m.txt", ios::in | ios::out);
    cudaDeviceSetLimit(cudaLimitStackSize, 256 * 1024);
    go_cal<<<1024,256>>>();
    cudaDeviceSynchronize();
    cudaMemcpy(host_array, device_array, x * sizeof(unsigned long long), cudaMemcpyDeviceToHost);
    for(int i=2;i<10000;i++){
        for(int j=1;j<1000;j++){
            if(all_road[i][j]!=0){
                f << all_road[i][j] << ",";
            }
            else{break;}
        }
        f << endl;
    }
    f.close();
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