// Cairo 2.x StarkNet Contract Example
// Fibonacci computation as a smart contract

#[starknet::interface]
trait IFibonacci<TContractState> {
    fn compute_fib(self: @TContractState, n: felt252) -> felt252;
    fn compute_fib_pair(self: @TContractState, n: felt252) -> (felt252, felt252);
}

#[starknet::contract]
mod FibonacciContract {
    use cairo_fibonacci::{fib_iterative, fib_pair};

    #[storage]
    struct Storage {
        last_computed_n: felt252,
        last_result: felt252,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.last_computed_n.write(0);
        self.last_result.write(0);
    }

    #[abi(embed_v0)]
    impl FibonacciImpl of super::IFibonacci<ContractState> {
        fn compute_fib(self: @ContractState, n: felt252) -> felt252 {
            let result = fib_iterative(n);
            result
        }

        fn compute_fib_pair(self: @ContractState, n: felt252) -> (felt252, felt252) {
            fib_pair(n)
        }
    }

    #[generate_trait]
    impl InternalFunctions of InternalFunctionsTrait {
        fn _store_result(ref self: ContractState, n: felt252, result: felt252) {
            self.last_computed_n.write(n);
            self.last_result.write(result);
        }
    }

    #[external(v0)]
    fn get_last_computed(self: @ContractState) -> (felt252, felt252) {
        (self.last_computed_n.read(), self.last_result.read())
    }
}

