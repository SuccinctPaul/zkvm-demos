/* 
 * Multi-Program Dispatcher in C for MIPS architecture
 * This program will be compiled to MIPS and proven by o1vm
 */

#include <stdint.h>

// Algorithms
uint32_t fibonacci(uint32_t n) {
    if (n <= 1) return n;
    uint32_t a = 0, b = 1, temp;
    for (uint32_t i = 2; i <= n; i++) {
        temp = a + b; a = b; b = temp;
    }
    return b;
}

uint32_t factorial(uint32_t n) {
    uint32_t res = 1;
    for (uint32_t i = 2; i <= n; i++) res *= i;
    return res;
}

uint32_t sum_n(uint32_t n) {
    return n * (n + 1) / 2;
}

// Mock Input Functions
// In a real MIPS zkVM, these would read from memory mapped I/O or registers
// populated by the host. For this demo, we hardcode or allow simple modification.
uint32_t get_program_id() { 
    return 0; // 0=Fib, 1=Sum, 2=Factorial
} 

uint32_t get_input_n() { 
    return 10; 
}

// Main entry point
int main() {
    uint32_t id = get_program_id();
    uint32_t n = get_input_n();
    uint32_t result = 0;

    switch(id) {
        case 0: result = fibonacci(n); break;
        case 1: result = sum_n(n); break;
        case 2: result = factorial(n); break;
        default: result = 0; break;
    }
    
    return result;
}
