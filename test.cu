#include <stdio.h>

__global__ void kernel() {
    printf("Hello from thread %d\n", threadIdx.x);
}

int main() {
    kernel<<<1, 5>>>();
    cudaDeviceSynchronize();
    return 0;
}
