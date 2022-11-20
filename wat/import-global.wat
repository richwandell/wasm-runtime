(module
    (import "hello" "world" (global $hello_world i32))
    (import "hello1" "world1" (global $hello_world1 (mut i32)))
)