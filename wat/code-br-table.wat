(module
    (func (export "nop")
          (i32.const 1)  ;; value used to select a branch
          (br_table 0 0 0))
)