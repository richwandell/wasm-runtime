(module
  (import "js" "tbl" (table 2 anyfunc))
  (memory (import "js" "mem") 1)
  (func $f42 (result i32) i32.const 42)
  (func $f83 (result i32) i32.const 83)
  (elem (i32.const 0) $f42 $f83)
)
