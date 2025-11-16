/* 
 * Fibonacci computation in C for MIPS architecture
 * This program will be compiled to MIPS and proven by o1vm
 */

#include <stdint.h>

// Simple Fibonacci implementation for MIPS
uint32_t fibonacci(uint32_t n) {
    if (n <= 1) {
        return n;
    }
    
    uint32_t a = 0;
    uint32_t b = 1;
    uint32_t temp;
    
    for (uint32_t i = 2; i <= n; i++) {
        temp = a + b;
        a = b;
        b = temp;
    }
    
    return b;
}

// Main entry point
int main() {
    // Compute fibonacci(10)
    uint32_t n = 10;
    uint32_t result = fibonacci(n);
    
    // Return the result
    return result;
}

