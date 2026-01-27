(component
  (core module $m
    (func $startup (export "startup") (result i32)
      i32.const 1
    )
  )
  (core instance $i (instantiate $m))
  (func $f (export "startup") (result u32) (canon lift (core func $i "startup")))
  (instance $inst
    (export "startup" (func $f))
  )
  (export "test:root/root" (instance $inst))
)
