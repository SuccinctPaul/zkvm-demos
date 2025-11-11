/*
 * Fibonacci calculation for Valida zkVM
 * 
 * This is a guest program that runs inside the Valida zkVM.
 * It computes the nth Fibonacci number.
 */

#include <stdio.h>
#include <stdint.h>

// Recursive fibonacci implementation (matching other zkVM demos)
uint32_t fibonacci(uint32_t n) {
    if (n == 0) return 1;
    if (n == 1) return 1;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
    // In Valida, input would typically be read from a predefined location
    // For this demo, we'll use a hardcoded value that can be changed via compilation
    uint32_t n = 10; // This value will be injected by the host program
    
    printf("Computing Fibonacci(%u)...\n", n);
    uint32_t result = fibonacci(n);
    printf("Fibonacci(%u) = %u\n", n, result);
    
    return 0;
}

