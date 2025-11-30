;; IsPrime function in WebAssembly Text Format
;; Returns 1 if n is prime, 0 otherwise
;; For use with zkEngine/NovaNet

(module
  ;; Export the isprime function
  (func (export "isprime") (param $n i32) (result i32)
    (local $i i32)
    (local $limit i32)
    
    ;; Handle n <= 1: not prime
    (if (i32.le_s (local.get $n) (i32.const 1))
      (then (return (i32.const 0)))
    )
    
    ;; Handle n == 2: prime
    (if (i32.eq (local.get $n) (i32.const 2))
      (then (return (i32.const 1)))
    )
    
    ;; Handle even numbers: not prime
    (if (i32.eq (i32.rem_s (local.get $n) (i32.const 2)) (i32.const 0))
      (then (return (i32.const 0)))
    )
    
    ;; Check odd divisors up to sqrt(n)
    ;; We'll check up to n/2 for simplicity (less efficient but works)
    (local.set $i (i32.const 3))
    (local.set $limit (i32.div_s (local.get $n) (i32.const 2)))
    
    (block $not_prime
      (loop $check
        ;; If i > limit, number is prime
        (br_if $not_prime (i32.gt_s (local.get $i) (local.get $limit)))
        
        ;; If n % i == 0, not prime
        (if (i32.eq (i32.rem_s (local.get $n) (local.get $i)) (i32.const 0))
          (then (return (i32.const 0)))
        )
        
        ;; i += 2 (check odd numbers only)
        (local.set $i (i32.add (local.get $i) (i32.const 2)))
        (br $check)
      )
    )
    
    ;; Number is prime
    (i32.const 1)
  )
)

