
#[macro_export]
macro_rules! now {
    () => {
        SystemTime::now()
    };
}