pub fn Test_Bench_gc() -> crate::UnknownType {
    // Rust has no manually triggered full GC; the benchmark only needs a
    // consistent starting point between runs.
    crate::Value::Func1(purust_core::Func1::Static(|_| crate::Value::Unit))
}
