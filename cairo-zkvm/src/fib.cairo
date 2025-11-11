%builtins output

from starkware.cairo.common.serialize import serialize_word

// Compute the nth Fibonacci number recursively
func fib(n) -> (res: felt) {
    if (n == 0) {
        return (res=0);
    }
    if (n == 1) {
        return (res=1);
    }
    
    let (a) = fib(n - 1);
    let (b) = fib(n - 2);
    return (res=a + b);
}

func main{output_ptr: felt*}() {
    alloc_locals;
    
    // Calculate Fibonacci for n = 10
    let n = 10;
    let (result) = fib(n);
    
    // Output: first the input value, then the result
    serialize_word(n);
    serialize_word(result);
    
    return ();
}

