(module
  (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (import "wasi_unstable" "fd_read" (func $fd_read (param i32 i32 i32 i32) (result i32)))
  (memory 1)
  (export "memory" (memory 0))

  (data (i32.const 16) "hello")
  (data (i32.const 22) "world")
  (func $start (export "_start")
    (i32.store (i32.const 0) (i32.const 16)) ;; iov_base "hello"
    (i32.store (i32.const 4) (i32.const 5)) ;;iov_len "hello"
    (i32.store (i32.const 8) (i32.const 22)) ;; iov_base "world"
    (i32.store (i32.const 12) (i32.const 5)) ;; iov_len "world"

    (call $fd_write
        (i32.const 1)
        (i32.const 0) ;; list of IO vectors starts on index 0
        (i32.const 2) ;; we have two IO vectors (two strings)
        (i32.const 27) ;; 22 + 5 = 27
    )
    drop
  )
)
