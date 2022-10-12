(module
  (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (import "wasi_unstable" "fd_read" (func $fd_read (param i32 i32 i32 i32) (result i32)))
  (memory 1)
  (export "memory" (memory 0))
  (data (i32.const 8) "hello world")
  (data (i32.const 19) "hi rich")

  (func $start
    ;; data starts on 8th byte in memory
    (i32.store (i32.const 0) (i32.const 8))

    ;; data "hello\n" has length of 6 bytes
    (i32.store (i32.const 4) (i32.const 11))

    (call $fd_write
      (i32.const 1)  ;; stdout file_descriptor
      (i32.const 0)  ;; memory index of the list io vectors
      (i32.const 1)  ;; length of the list 1 string to print
      (i32.const 14) ;; nwritten
    )
    drop
  )
  (export "_start" (func $start))
)