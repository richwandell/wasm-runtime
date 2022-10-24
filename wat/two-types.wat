(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add
  )
  (export "add" (func $add))
  (func $add2 (param $lhs i64) (param $rhs i64) (result i64)
    local.get $lhs
    local.get $rhs
    i64.add
  )
  (export "add2" (func $add2))
)