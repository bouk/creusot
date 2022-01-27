fn closure_param<F : Fn(u32)>(f: F) {
  (f)(0)
}