(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add
  )

  (func $sub (param $lhs f32) (param $rhs f32) (result f32)
    local.get $lhs
    local.get $rhs
    f32.sub
  )
)