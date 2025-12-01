;; Popcount function in WebAssembly Text Format
;; Counts the number of 1 bits in n
;; For use with zkEngine/NovaNet

(module
  ;; Export the popcount function
  (func (export "popcount") (param $n i32) (result i32)
    (local $count i32)
    
    ;; Initialize count = 0
    (local.set $count (i32.const 0))
    
    ;; Loop while n != 0
    (block $done
      (loop $count_bits
        ;; If n == 0, we're done
        (br_if $done (i32.eq (local.get $n) (i32.const 0)))
        
        ;; count += n & 1
        (local.set $count 
          (i32.add 
            (local.get $count) 
            (i32.and (local.get $n) (i32.const 1))
          )
        )
        
        ;; n = n >> 1 (unsigned shift)
        (local.set $n (i32.shr_u (local.get $n) (i32.const 1)))
        
        (br $count_bits)
      )
    )
    
    (local.get $count)
  )
)

