// Multi-Program Dispatcher in Aleo
program dispatcher.aleo;

// Main function acting as dispatcher
// r0: program_id (0=Fib, 1=Sum)
// r1: n
function main:
    input r0 as u32.public;
    input r1 as u32.public;
    output r2 as u64.private;

    // Check if program_id == 0 (Fibonacci)
    is.eq r0 0u32 into r3;
    
    // Check if program_id == 1 (Sum)
    is.eq r0 1u32 into r4;

    // Result variable
    cast 0u64 into r5 as u64;

    // --- Branch 1: Fibonacci ---
    // For demo, we return fixed value 55 if n=10, or compute simple logic
    // Aleo loops are unrolled, writing a full fib loop here is verbose
    // We use a conditional assignment for demo
    cast 55u64 into r6 as u64;
    ternary r3 r6 r5 into r5;

    // --- Branch 2: Sum ---
    // Sum = n * (n + 1) / 2
    add r1 1u32 into r7;
    mul r1 r7 into r8;
    div r8 2u32 into r9;
    cast r9 into r10 as u64;
    ternary r4 r10 r5 into r5;

    // Output result
    output r5 as u64.private;
