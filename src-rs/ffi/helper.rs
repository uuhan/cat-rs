use libc::{gettimeofday, malloc, timeval};
use std::mem;
use std::ptr;

pub unsafe fn GetTime64() -> usize {
    let buf: usize;
    let mut tv: timeval = mem::uninitialized();
    gettimeofday(&mut tv, ptr::null_mut());
    buf = (tv.tv_sec * 1000 + (tv.tv_usec / 1000) as (i64)) as (usize);
    buf
}
