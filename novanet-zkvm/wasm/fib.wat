;; Fibonacci function in WebAssembly Text Format
;; For use with zkEngine/NovaNet

(module
  ;; Export the fibonacci function
  (func (export "fib") (param $n i32) (result i32)
    (local $a i32)
    (local $b i32)
    (local $temp i32)
    (local $i i32)
    
    ;; Handle base cases
    (if (i32.le_s (local.get $n) (i32.const 1))
      (then (return (local.get $n)))
    )
    
    ;; Initialize: a = 0, b = 1
    (local.set $a (i32.const 0))
    (local.set $b (i32.const 1))
    (local.set $i (i32.const 2))
    
    ;; Loop: compute fib(n)
    (block $break
      (loop $continue
        ;; temp = a + b
        (local.set $temp (i32.add (local.get $a) (local.get $b)))
        ;; a = b
        (local.set $a (local.get $b))
        ;; b = temp
        (local.set $b (local.get $temp))
        ;; i++
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        ;; if i <= n, continue
        (br_if $continue (i32.le_s (local.get $i) (local.get $n)))
      )
    )
    
    (local.get $b)
  )
)

