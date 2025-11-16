// Fibonacci program in Aleo
// This demonstrates a simple computation in snarkVM

program fibonacci.aleo;

// Compute the nth Fibonacci number
function compute:
    input r0 as u32.public;
    output r1 as u64.private;
    
    // Initialize fibonacci sequence
    cast 0u64 into r1 as u64;  // prev = 0
    cast 1u64 into r2 as u64;  // curr = 1
    cast 0u32 into r3 as u32;  // counter = 0
    
    // Loop to compute fibonacci
    loop:
        // Check if counter < n
        lt r3 r0 into r4;
        branch.eq r4 false to done;
        
        // temp = curr
        cast r2 into r5 as u64;
        
        // curr = prev + curr
        add r1 r2 into r2;
        
        // prev = temp
        cast r5 into r1 as u64;
        
        // counter++
        add r3 1u32 into r3;
        
        branch to loop;
    
    done:
        // Return prev (which is now fib(n))
        cast r1 into r1 as u64;

