use std::time::SystemTime;

#[macro_export]
macro_rules! now {
    () => {
        SystemTime::now()
    };
}