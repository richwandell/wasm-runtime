(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add
  )
  (func $sub (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.sub
  )
  (memory 1)
  (export "add" (func $add))
  (export "sub" (func $sub))
  (export "memory" (memory 0))
)