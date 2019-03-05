#[macro_export]
macro_rules! c {
    ($data:ident) => {
        CString::new($data).unwrap().as_ptr() as *const u8
    };
    ($expr:expr) => {
        CString::new($expr).unwrap().as_ptr() as *const u8
    };
}
