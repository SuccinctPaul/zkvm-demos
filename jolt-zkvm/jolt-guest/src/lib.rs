#![cfg_attr(feature = "guest", no_std)]

use zkvm_programs::execute_program as common_execute;

/// Execute a selected program by ID
/// This function will be proven by Jolt zkVM
///
/// Note: The max_trace_length needs to be large enough for the computation.
/// - fibonacci(50): ~65536 is sufficient
/// - hash(1000): may need more trace length
/// - signature: needs even more due to ECDSA operations
#[jolt::provable(memory_size = 10240, max_trace_length = 1048576)]
fn execute_program(id: u32, n: u32) -> u32 {
    common_execute(id, n)
}
