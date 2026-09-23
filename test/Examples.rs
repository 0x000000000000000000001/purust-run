use std::time::Duration;

pub fn Test_Examples_setTimeout(milliseconds: i64, callback: crate::UnknownType) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |_| {
        let callback = callback.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(milliseconds.max(0) as u64));
            callback.unwrap_func1()(crate::Value::Unit);
        });
        crate::Value::Unit
    })))
}
