(module
  (import "func_1" "one"
      (func $fd_read
        (param i32 i32 i32 i32)
        (result i32)))

  (import "func_2" "two"
      (func $fd_write
        (param i32)
        (result i32)))
)