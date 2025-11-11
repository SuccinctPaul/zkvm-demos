%builtins output

from starkware.cairo.common.serialize import serialize_word

func main{output_ptr: felt*}() {
    // Output a simple message: 42 (the answer to everything)
    serialize_word(42);
    
    return ();
}


