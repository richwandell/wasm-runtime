(module
    (func $func (param $num i32)
        (block $my_block

          ;; $num is equal to 100
          local.get $num
          i32.const 100
          i32.eq

          (if
            (then
              ;; branch to the end of the block
              br $my_block
            )
          )

          ;; not reachable when $num is 100
          local.get $num
          drop
        )
    )
    (func $func1 (param $num i32)
        (loop $my_loop

          ;; add one to $i
          local.get $num
          i32.const 1
          i32.add
          local.set $num

          ;; log the current value of $i
          local.get $num
          drop

          ;; if $i is less than 10 branch to loop
          local.get $num
          i32.const 10
          i32.lt_s
          br_if $my_loop
        )
    )
    (func (export "nop")
      (i32.const 1)  ;; value used to select a branch
      (br_table 0))
)