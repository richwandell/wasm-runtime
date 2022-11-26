(module
    (func $start (param $p i32)
        local.get $p
        i32.const 5
        i32.add
    )
    (start $start)
)