;; Factorial function in WebAssembly Text Format
;; Computes factorial(n) = n!
;; For use with zkEngine/NovaNet

(module
  ;; Export the factorial function
  (func (export "factorial") (param $n i32) (result i32)
    (local $result i32)
    (local $i i32)
    
    ;; Handle base case
    (if (i32.le_s (local.get $n) (i32.const 1))
      (then (return (i32.const 1)))
    )
    
    ;; Initialize: result = 1, i = 2
    (local.set $result (i32.const 1))
    (local.set $i (i32.const 2))
    
    ;; Loop: compute n!
    (block $break
      (loop $continue
        ;; result = result * i
        (local.set $result (i32.mul (local.get $result) (local.get $i)))
        ;; i++
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        ;; if i <= n, continue
        (br_if $continue (i32.le_s (local.get $i) (local.get $n)))
      )
    )
    
    (local.get $result)
  )
)

