#[macro_export]
macro_rules! c {
    ($data:ident) => {
        std::ffi::CString::new($data).unwrap().as_ptr()
    };
    ($expr:expr) => {
        std::ffi::CString::new($expr).unwrap().as_ptr()
    };
}
