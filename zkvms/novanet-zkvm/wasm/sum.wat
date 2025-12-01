;; Sum function in WebAssembly Text Format
;; Computes sum(n) = 1 + 2 + ... + n = n*(n+1)/2
;; For use with zkEngine/NovaNet

(module
  ;; Export the sum function
  (func (export "sum") (param $n i32) (result i32)
    ;; Return n * (n + 1) / 2
    (i32.div_s
      (i32.mul
        (local.get $n)
        (i32.add (local.get $n) (i32.const 1))
      )
      (i32.const 2)
    )
  )
)

