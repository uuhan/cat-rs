use libc::{
    c_char, free, isprint, isspace, malloc, memcmp, memcpy, memset, realloc, strchr, strlen,
    tolower, toupper,
};
use std::mem;

extern "C" {
    fn calloc(__count: usize, __size: usize) -> *mut ::std::os::raw::c_void;
    fn catsdsavail(s: *mut u8) -> usize;
    fn catsdscatprintf(s: *mut u8, fmt: *const u8, ...) -> *mut u8;
    fn catsdslen(s: *mut u8) -> usize;
}

#[no_mangle]
pub unsafe extern "C" fn isascii(mut _c: i32) -> i32 {
    (_c & !0x7fi32 == 0i32) as (i32)
}

#[derive(Copy)]
#[repr(C)]
pub struct sdshdr {
    pub len: u32,
    pub free: u32,
    pub buf: *mut c_char,
}

impl Clone for sdshdr {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdsnewlen(
    mut init: *const ::std::os::raw::c_void,
    mut initlen: usize,
) -> *mut u8 {
    let mut sh: *mut sdshdr;
    if !init.is_null() {
        sh = malloc(
            ::std::mem::size_of::<sdshdr>()
                .wrapping_add(initlen)
                .wrapping_add(1usize),
        ) as (*mut sdshdr);
    } else {
        sh = calloc(
            ::std::mem::size_of::<sdshdr>()
                .wrapping_add(initlen)
                .wrapping_add(1usize),
            1usize,
        ) as (*mut sdshdr);
    }
    if sh == 0i32 as (*mut ::std::os::raw::c_void) as (*mut sdshdr) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*sh).len = initlen as (u32);
        (*sh).free = 0u32;
        0i32 as (*mut u8)
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdsnewEmpty(mut preAlloclen: usize) -> *mut u8 {
    let mut sh: *mut sdshdr;
    sh = malloc(
        ::std::mem::size_of::<sdshdr>()
            .wrapping_add(preAlloclen)
            .wrapping_add(1usize),
    ) as (*mut sdshdr);
    if sh == 0i32 as (*mut ::std::os::raw::c_void) as (*mut sdshdr) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*sh).len = 0u32;
        (*sh).free = preAlloclen as (u32);
        0i32 as (*mut u8)
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdsempty() -> *mut u8 {
    catsdsnewlen((*b"\0").as_ptr() as (*const ::std::os::raw::c_void), 0usize)
}

#[no_mangle]
pub unsafe extern "C" fn catsdsnew(mut init: *const i8) -> *mut u8 {
    let mut initlen: usize = if init == 0i32 as (*mut ::std::os::raw::c_void) as (*const i8) {
        0usize
    } else {
        strlen(init)
    };
    catsdsnewlen(init as (*const ::std::os::raw::c_void), initlen)
}

#[no_mangle]
pub unsafe extern "C" fn catsdsdup(s: *mut u8) -> *mut u8 {
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        catsdsnewlen(s as (*const ::std::os::raw::c_void), catsdslen(s))
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdsfree(mut s: *mut u8) {
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
    } else {
        free(s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
            as (*mut ::std::os::raw::c_void));
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdsupdatelen(mut s: *mut u8) {
    let mut sh: *mut sdshdr = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
        as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
    let mut reallen: i32 = strlen(s as (*const i8)) as (i32);
    (*sh).free = (*sh)
        .free
        .wrapping_add((*sh).len.wrapping_sub(reallen as (u32)));
    (*sh).len = reallen as (u32);
}

#[no_mangle]
pub unsafe extern "C" fn catsdsclear(mut s: *mut u8) {
    let mut sh: *mut sdshdr = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
        as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
    (*sh).free = (*sh).free.wrapping_add((*sh).len);
    (*sh).len = 0u32;
    // sh->buf[0] = '\0'
}

#[no_mangle]
pub unsafe extern "C" fn catsdsMakeRoomFor(mut s: *mut u8, mut addlen: usize) -> *mut u8 {
    let mut sh: *mut sdshdr;
    let mut newsh: *mut sdshdr;
    let mut free: usize = catsdsavail(s);
    let mut len: usize;
    let mut newlen: usize;
    if free >= addlen {
        s
    } else {
        len = catsdslen(s);
        sh = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
            as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
        newlen = len.wrapping_add(addlen);
        if newlen < (1024i32 * 1024i32) as (usize) {
            newlen = newlen.wrapping_mul(2usize);
        } else {
            newlen = newlen.wrapping_add((1024i32 * 1024i32) as (usize));
        }
        newsh = realloc(
            sh as (*mut ::std::os::raw::c_void),
            ::std::mem::size_of::<sdshdr>()
                .wrapping_add(newlen)
                .wrapping_add(1usize),
        ) as (*mut sdshdr);
        (if newsh == 0i32 as (*mut ::std::os::raw::c_void) as (*mut sdshdr) {
            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        } else {
            (*newsh).free = newlen.wrapping_sub(len) as (u32);
            // return newsh->buf;
            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdsRemoveFreeSpace(mut s: *mut u8) -> *mut u8 {
    let mut sh: *mut sdshdr;
    sh = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize))) as (*mut ::std::os::raw::c_void)
        as (*mut sdshdr);
    sh = realloc(
        sh as (*mut ::std::os::raw::c_void),
        ::std::mem::size_of::<sdshdr>()
            .wrapping_add((*sh).len as (usize))
            .wrapping_add(1usize),
    ) as (*mut sdshdr);
    (*sh).free = 0u32;
    // return sh->buf
    0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
}

#[no_mangle]
pub unsafe extern "C" fn catsdsAllocSize(mut s: *mut u8) -> usize {
    let mut sh: *mut sdshdr = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
        as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
    ::std::mem::size_of::<sdshdr>()
        .wrapping_add((*sh).len as (usize))
        .wrapping_add((*sh).free as (usize))
        .wrapping_add(1usize)
}

#[no_mangle]
pub unsafe extern "C" fn catsdsIncrLen(mut s: *mut u8, mut incr: i32) {
    let mut sh: *mut sdshdr = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
        as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
    (*sh).len = (*sh).len.wrapping_add(incr as (u32));
    (*sh).free = (*sh).free.wrapping_sub(incr as (u32));
    *s.offset((*sh).len as (isize)) = b'\0';
}

#[no_mangle]
pub unsafe extern "C" fn catsdsgrowzero(mut s: *mut u8, mut len: usize) -> *mut u8 {
    let mut sh: *mut sdshdr = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
        as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
    let mut totlen: usize;
    let mut curlen: usize = (*sh).len as (usize);
    if len <= curlen {
        s
    } else {
        s = catsdsMakeRoomFor(s, len.wrapping_sub(curlen));
        (if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        } else {
            sh = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
                as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
            memset(
                s.offset(curlen as (isize)) as (*mut ::std::os::raw::c_void),
                0i32,
                len.wrapping_sub(curlen).wrapping_add(1usize),
            );
            totlen = (*sh).len.wrapping_add((*sh).free) as (usize);
            (*sh).len = len as (u32);
            (*sh).free = totlen.wrapping_sub((*sh).len as (usize)) as (u32);
            s
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdscatlen(
    mut s: *mut u8,
    mut t: *const ::std::os::raw::c_void,
    mut len: usize,
) -> *mut u8 {
    let mut sh: *mut sdshdr;
    let mut curlen: usize = catsdslen(s);
    s = catsdsMakeRoomFor(s, len);
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        sh = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
            as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
        memcpy(
            s.offset(curlen as (isize)) as (*mut ::std::os::raw::c_void),
            t,
            len,
        );
        (*sh).len = curlen.wrapping_add(len) as (u32);
        (*sh).free = ((*sh).free as (usize)).wrapping_sub(len) as (u32);
        *s.offset(curlen.wrapping_add(len) as (isize)) = b'\0';
        s
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdscatchar(mut s: *mut u8, mut c: u8) -> *mut u8 {
    let mut sh: *mut sdshdr;
    let mut curlen: usize = catsdslen(s);
    s = catsdsMakeRoomFor(s, 1usize);
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        sh = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
            as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
        *s.offset(curlen as (isize)) = c;
        *s.offset(curlen.wrapping_add(1usize) as (isize)) = b'\0';
        (*sh).len = (*sh).len.wrapping_add(1u32);
        (*sh).free = (*sh).free.wrapping_sub(1u32);
        s
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdscat(mut s: *mut u8, mut t: *const u8) -> *mut u8 {
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        || t == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
    {
        s
    } else {
        catsdscatlen(
            s,
            t as (*const ::std::os::raw::c_void),
            strlen(t as *const i8),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdscatsds(mut s: *mut u8, t: *mut u8) -> *mut u8 {
    catsdscatlen(s, t as (*const ::std::os::raw::c_void), catsdslen(t))
}

#[no_mangle]
pub unsafe extern "C" fn catsdscpylen(mut s: *mut u8, mut t: *const u8, mut len: usize) -> *mut u8 {
    let mut sh: *mut sdshdr = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
        as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
    let mut totlen: usize = (*sh).free.wrapping_add((*sh).len) as (usize);
    if totlen < len {
        s = catsdsMakeRoomFor(s, len.wrapping_sub((*sh).len as (usize)));
        if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            return 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        } else {
            sh = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
                as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
            totlen = (*sh).free.wrapping_add((*sh).len) as (usize);
        }
    }
    memcpy(
        s as (*mut ::std::os::raw::c_void),
        t as (*const ::std::os::raw::c_void),
        len,
    );
    *s.offset(len as (isize)) = b'\0';
    (*sh).len = len as (u32);
    (*sh).free = totlen.wrapping_sub(len) as (u32);
    s
}

#[no_mangle]
pub unsafe extern "C" fn catsdscpy(mut s: *mut u8, mut t: *const u8) -> *mut u8 {
    catsdscpylen(s, t, strlen(t as *const i8))
}

#[no_mangle]
pub unsafe extern "C" fn sdsll2str(mut s: *mut u8, mut value: isize) -> i32 {
    let mut p: *mut u8;
    let mut aux: u8;
    let mut v: usize;
    let mut l: usize;
    v = if value < 0isize { -value } else { value } as (usize);
    p = s;
    'loop1: loop {
        *{
            let _old = p;
            p = p.offset(1isize);
            _old
        } = (b'0' as (usize)).wrapping_add(v.wrapping_rem(10usize)) as (u8);
        v = v.wrapping_div(10usize);
        if v == 0 {
            break;
        }
    }
    if value < 0isize {
        *{
            let _old = p;
            p = p.offset(1isize);
            _old
        } = b'-';
    }
    l = ((p as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize))
        as (usize);
    *p = b'\0';
    p = p.offset(-1isize);
    'loop6: loop {
        if !(s < p) {
            break;
        }
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1isize);
        p = p.offset(-1isize);
    }
    l as (i32)
}

#[no_mangle]
pub unsafe extern "C" fn sdsull2str(mut s: *mut u8, mut v: usize) -> i32 {
    let mut p: *mut u8;
    let mut aux: u8;
    let mut l: usize;
    p = s;
    'loop1: loop {
        *{
            let _old = p;
            p = p.offset(1isize);
            _old
        } = (b'0' as (usize)).wrapping_add(v.wrapping_rem(10usize)) as (u8);
        v = v.wrapping_div(10usize);
        if v == 0 {
            break;
        }
    }
    l = ((p as (isize)).wrapping_sub(s as (isize)) / ::std::mem::size_of::<u8>() as (isize))
        as (usize);
    *p = b'\0';
    p = p.offset(-1isize);
    'loop4: loop {
        if !(s < p) {
            break;
        }
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1isize);
        p = p.offset(-1isize);
    }
    l as (i32)
}

#[no_mangle]
pub unsafe extern "C" fn catsdsfromlonglong(mut value: isize) -> *mut u8 {
    let mut buf: [u8; 21] = mem::uninitialized();
    let mut len: i32 = sdsll2str(buf.as_mut_ptr(), value);
    catsdsnewlen(
        buf.as_mut_ptr() as (*const ::std::os::raw::c_void),
        len as (usize),
    )
}

#[no_mangle]
pub unsafe extern "C" fn catsdstrim(mut s: *mut u8, mut cset: *const i8) -> *mut u8 {
    let mut sh: *mut sdshdr = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
        as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
    let mut start: *mut u8;
    let mut end: *mut u8;
    let mut sp: *mut u8;
    let mut ep: *mut u8;
    let mut len: usize;
    sp = {
        start = s;
        start
    };
    ep = {
        end = s.offset(catsdslen(s) as (isize)).offset(-1isize);
        end
    };
    'loop1: loop {
        if !(sp <= end && !strchr(cset, *sp as (i32)).is_null()) {
            break;
        }
        sp = sp.offset(1isize);
    }
    'loop2: loop {
        if !(ep > start && !strchr(cset, *ep as (i32)).is_null()) {
            break;
        }
        ep = ep.offset(-1isize);
    }
    len = if sp > ep {
        0isize
    } else {
        (ep as (isize)).wrapping_sub(sp as (isize)) / ::std::mem::size_of::<u8>() as (isize)
            + 1isize
    } as (usize);
    // if (sh->buf != sp) memmove(sh->buf, sp, len);
    // sh-buf[len] = '\0'
    (*sh).free =
        ((*sh).free as (usize)).wrapping_add(((*sh).len as (usize)).wrapping_sub(len)) as (u32);
    (*sh).len = len as (u32);
    s
}

#[no_mangle]
pub unsafe extern "C" fn catsdsrange(mut s: *mut u8, mut start: i32, mut end: i32) {
    let mut sh: *mut sdshdr = s.offset(-(::std::mem::size_of::<sdshdr>() as (isize)))
        as (*mut ::std::os::raw::c_void) as (*mut sdshdr);
    let mut newlen: usize;
    let mut len: usize = catsdslen(s);
    if len == 0usize {
    } else {
        if start < 0i32 {
            start = len.wrapping_add(start as (usize)) as (i32);
            if start < 0i32 {
                start = 0i32;
            }
        }
        if end < 0i32 {
            end = len.wrapping_add(end as (usize)) as (i32);
            if end < 0i32 {
                end = 0i32;
            }
        }
        newlen = if start > end {
            0i32
        } else {
            end - start + 1i32
        } as (usize);
        if newlen != 0usize {
            if start >= len as (i32) {
                newlen = 0usize;
            } else if end >= len as (i32) {
                end = len.wrapping_sub(1usize) as (i32);
                newlen = if start > end {
                    0i32
                } else {
                    end - start + 1i32
                } as (usize);
            }
        } else {
            start = 0i32;
        }
        // if (start && newlen) memmove(sh->buf, sh->buf + start, newlen);
        // sh->buf[newlen] = 0;
        (*sh).free = ((*sh).free as (usize))
            .wrapping_add(((*sh).len as (usize)).wrapping_sub(newlen))
            as (u32);
        (*sh).len = newlen as (u32);
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdstolower(mut s: *mut u8) {
    let mut len: i32 = catsdslen(s) as (i32);
    let mut j: i32;
    j = 0i32;
    'loop1: loop {
        if !(j < len) {
            break;
        }
        *s.offset(j as (isize)) = tolower(*s.offset(j as (isize)) as (i32)) as (u8);
        j = j + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdstoupper(mut s: *mut u8) {
    let mut len: i32 = catsdslen(s) as (i32);
    let mut j: i32;
    j = 0i32;
    'loop1: loop {
        if !(j < len) {
            break;
        }
        *s.offset(j as (isize)) = toupper(*s.offset(j as (isize)) as (i32)) as (u8);
        j = j + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdscmp(s1: *mut u8, s2: *mut u8) -> i32 {
    let mut l1: usize;
    let mut l2: usize;
    let mut minlen: usize;
    let mut cmp: i32;
    l1 = catsdslen(s1);
    l2 = catsdslen(s2);
    minlen = if l1 < l2 { l1 } else { l2 };
    cmp = memcmp(
        s1 as (*const ::std::os::raw::c_void),
        s2 as (*const ::std::os::raw::c_void),
        minlen,
    );
    if cmp == 0i32 {
        l1.wrapping_sub(l2) as (i32)
    } else {
        cmp
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdssplitlen(
    mut s: *const u8,
    mut len: i32,
    mut sep: *const u8,
    mut seplen: i32,
    mut count: *mut i32,
) -> *mut *mut u8 {
    let mut _currentBlock;
    let mut elements: i32 = 0i32;
    let mut slots: i32 = 5i32;
    let mut start: i32 = 0i32;
    let mut j: i32;
    let mut tokens: *mut *mut u8;
    if seplen < 1i32 || len < 0i32 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8)
    } else {
        tokens = malloc(::std::mem::size_of::<*mut u8>().wrapping_mul(slots as (usize)))
            as (*mut *mut u8);
        (if tokens == 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) {
            0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8)
        } else if len == 0i32 {
            *count = 0i32;
            tokens
        } else {
            j = 0i32;
            'loop4: loop {
                if !(j < len - (seplen - 1i32)) {
                    _currentBlock = 5;
                    break;
                }
                if slots < elements + 2i32 {
                    let mut newtokens: *mut *mut u8;
                    slots = slots * 2i32;
                    newtokens = realloc(
                        tokens as (*mut ::std::os::raw::c_void),
                        ::std::mem::size_of::<*mut u8>().wrapping_mul(slots as (usize)),
                    ) as (*mut *mut u8);
                    if newtokens == 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) {
                        _currentBlock = 14;
                        break;
                    }
                    tokens = newtokens;
                }
                if seplen == 1i32
                    && (*s.offset(j as (isize)) as (i32) == *sep.offset(0isize) as (i32))
                    || memcmp(
                        s.offset(j as (isize)) as (*const ::std::os::raw::c_void),
                        sep as (*const ::std::os::raw::c_void),
                        seplen as (usize),
                    ) == 0i32
                {
                    *tokens.offset(elements as (isize)) = catsdsnewlen(
                        s.offset(start as (isize)) as (*const ::std::os::raw::c_void),
                        (j - start) as (usize),
                    );
                    if *tokens.offset(elements as (isize))
                        == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                    {
                        _currentBlock = 14;
                        break;
                    }
                    elements = elements + 1;
                    start = j + seplen;
                    j = j + seplen - 1i32;
                }
                j = j + 1;
            }
            if _currentBlock == 5 {
                *tokens.offset(elements as (isize)) = catsdsnewlen(
                    s.offset(start as (isize)) as (*const ::std::os::raw::c_void),
                    (len - start) as (usize),
                );
                if !(*tokens.offset(elements as (isize))
                    == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8))
                {
                    elements = elements + 1;
                    *count = elements;
                    return tokens;
                }
            }
            let mut i: i32;
            i = 0i32;
            'loop15: loop {
                if !(i < elements) {
                    break;
                }
                catsdsfree(*tokens.offset(i as (isize)));
                i = i + 1;
            }
            free(tokens as (*mut ::std::os::raw::c_void));
            *count = 0i32;
            0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8)
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdsfreesplitres(mut tokens: *mut *mut u8, mut count: i32) {
    if tokens.is_null() {
    } else {
        'loop1: loop {
            if {
                let _old = count;
                count = count - 1;
                _old
            } == 0
            {
                break;
            }
            catsdsfree(*tokens.offset(count as (isize)));
        }
        free(tokens as (*mut ::std::os::raw::c_void));
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdscatrepr(
    mut s: *mut u8,
    mut p: *const u8,
    mut len: usize,
) -> *mut u8 {
    s = catsdscatlen(
        s,
        (*b"\"\0").as_ptr() as (*const ::std::os::raw::c_void),
        1usize,
    );
    'loop1: loop {
        if {
            let _old = len;
            len = len.wrapping_sub(1usize);
            _old
        } == 0
        {
            break;
        }
        let switch5 = *p;
        if switch5 as (i32) == b'\x08' as (i32) {
            s = catsdscatlen(
                s,
                (*b"\\b\0").as_ptr() as (*const ::std::os::raw::c_void),
                2usize,
            );
        } else if switch5 as (i32) == b'\x07' as (i32) {
            s = catsdscatlen(
                s,
                (*b"\\a\0").as_ptr() as (*const ::std::os::raw::c_void),
                2usize,
            );
        } else if switch5 as (i32) == b'\t' as (i32) {
            s = catsdscatlen(
                s,
                (*b"\\t\0").as_ptr() as (*const ::std::os::raw::c_void),
                2usize,
            );
        } else if switch5 as (i32) == b'\r' as (i32) {
            s = catsdscatlen(
                s,
                (*b"\\r\0").as_ptr() as (*const ::std::os::raw::c_void),
                2usize,
            );
        } else if switch5 as (i32) == b'\n' as (i32) {
            s = catsdscatlen(
                s,
                (*b"\\n\0").as_ptr() as (*const ::std::os::raw::c_void),
                2usize,
            );
        } else if switch5 as (i32) == b'\"' as (i32) || switch5 as (i32) == b'\\' as (i32) {
            s = catsdscatprintf(s, (*b"\\%c\0").as_ptr(), *p as (i32));
        } else if isprint(*p as (i32)) != 0 {
            s = catsdscatprintf(s, (*b"%c\0").as_ptr(), *p as (i32));
        } else {
            s = catsdscatprintf(s, (*b"\\x%02x\0").as_ptr(), *p as (i32));
        }
        p = p.offset(1isize);
    }
    catsdscatlen(
        s,
        (*b"\"\0").as_ptr() as (*const ::std::os::raw::c_void),
        1usize,
    )
}

#[no_mangle]
pub unsafe extern "C" fn is_hex_digit(mut c: u8) -> i32 {
    (c as (i32) >= b'0' as (i32) && (c as (i32) <= b'9' as (i32))
        || c as (i32) >= b'a' as (i32) && (c as (i32) <= b'f' as (i32))
        || c as (i32) >= b'A' as (i32) && (c as (i32) <= b'F' as (i32))) as (i32)
}

#[no_mangle]
pub unsafe extern "C" fn hex_digit_to_int(mut c: u8) -> i32 {
    if c as (i32) == b'F' as (i32) || c as (i32) == b'f' as (i32) {
        15i32
    } else if c as (i32) == b'E' as (i32) || c as (i32) == b'e' as (i32) {
        14i32
    } else if c as (i32) == b'D' as (i32) || c as (i32) == b'd' as (i32) {
        13i32
    } else if c as (i32) == b'C' as (i32) || c as (i32) == b'c' as (i32) {
        12i32
    } else if c as (i32) == b'B' as (i32) || c as (i32) == b'b' as (i32) {
        11i32
    } else if c as (i32) == b'A' as (i32) || c as (i32) == b'a' as (i32) {
        10i32
    } else if c as (i32) == b'9' as (i32) {
        9i32
    } else if c as (i32) == b'8' as (i32) {
        8i32
    } else if c as (i32) == b'7' as (i32) {
        7i32
    } else if c as (i32) == b'6' as (i32) {
        6i32
    } else if c as (i32) == b'5' as (i32) {
        5i32
    } else if c as (i32) == b'4' as (i32) {
        4i32
    } else if c as (i32) == b'3' as (i32) {
        3i32
    } else if c as (i32) == b'2' as (i32) {
        2i32
    } else if c as (i32) == b'1' as (i32) {
        1i32
    } else if c as (i32) == b'0' as (i32) {
        0i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdssplitargs(mut line: *const u8, mut argc: *mut i32) -> *mut *mut u8 {
    let mut _currentBlock;
    let mut p: *const u8 = line;
    let mut current: *mut u8 = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut vector: *mut *mut u8 = 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8);
    *argc = 0i32;
    'loop1: loop {
        if *p != 0 && (isspace(*p as (i32)) != 0) {
            p = p.offset(1isize);
        } else {
            if *p == 0 {
                _currentBlock = 3;
                break;
            }
            let mut inq: i32 = 0i32;
            let mut insq: i32 = 0i32;
            let mut done: i32 = 0i32;
            if current == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                current = catsdsempty();
            }
            'loop8: loop {
                if !(done == 0) {
                    break;
                }
                if inq != 0 {
                    if *p as (i32) == b'\\' as (i32)
                        && (*p.offset(1isize) as (i32) == b'x' as (i32))
                        && (is_hex_digit(*p.offset(2isize)) != 0)
                        && (is_hex_digit(*p.offset(3isize)) != 0)
                    {
                        let mut byte: u8;
                        byte = (hex_digit_to_int(*p.offset(2isize)) * 16i32
                            + hex_digit_to_int(*p.offset(3isize)))
                            as (u8);
                        current = catsdscatlen(
                            current,
                            &mut byte as (*mut u8) as (*const ::std::os::raw::c_void),
                            1usize,
                        );
                        p = p.offset(3isize);
                    } else if *p as (i32) == b'\\' as (i32) && (*p.offset(1isize) != 0) {
                        let mut c: u8;
                        p = p.offset(1isize);
                        let switch7 = *p;
                        if switch7 as (i32) == b'a' as (i32) {
                            c = b'\x07';
                        } else if switch7 as (i32) == b'b' as (i32) {
                            c = b'\x08';
                        } else if switch7 as (i32) == b't' as (i32) {
                            c = b'\t';
                        } else if switch7 as (i32) == b'r' as (i32) {
                            c = b'\r';
                        } else if switch7 as (i32) == b'n' as (i32) {
                            c = b'\n';
                        } else {
                            c = *p;
                        }
                        current = catsdscatlen(
                            current,
                            &mut c as (*mut u8) as (*const ::std::os::raw::c_void),
                            1usize,
                        );
                    } else if *p as (i32) == b'\"' as (i32) {
                        if *p.offset(1isize) != 0 && (isspace(*p.offset(1isize) as (i32)) == 0) {
                            _currentBlock = 34;
                            break 'loop1;
                        }
                        done = 1i32;
                    } else {
                        if *p == 0 {
                            _currentBlock = 34;
                            break 'loop1;
                        }
                        current =
                            catsdscatlen(current, p as (*const ::std::os::raw::c_void), 1usize);
                    }
                } else if insq != 0 {
                    if *p as (i32) == b'\\' as (i32)
                        && (*p.offset(1isize) as (i32) == b'\'' as (i32))
                    {
                        p = p.offset(1isize);
                        current = catsdscatlen(
                            current,
                            (*b"\'\0").as_ptr() as (*const ::std::os::raw::c_void),
                            1usize,
                        );
                    } else if *p as (i32) == b'\'' as (i32) {
                        if *p.offset(1isize) != 0 && (isspace(*p.offset(1isize) as (i32)) == 0) {
                            _currentBlock = 34;
                            break 'loop1;
                        }
                        done = 1i32;
                    } else {
                        if *p == 0 {
                            _currentBlock = 34;
                            break 'loop1;
                        }
                        current =
                            catsdscatlen(current, p as (*const ::std::os::raw::c_void), 1usize);
                    }
                } else {
                    let switch6 = *p;
                    if switch6 as (i32) == b'\'' as (i32) {
                        insq = 1i32;
                    } else if switch6 as (i32) == b'\"' as (i32) {
                        inq = 1i32;
                    } else if switch6 as (i32) == b'\0' as (i32)
                        || switch6 as (i32) == b'\t' as (i32)
                        || switch6 as (i32) == b'\r' as (i32)
                        || switch6 as (i32) == b'\n' as (i32)
                        || switch6 as (i32) == b' ' as (i32)
                    {
                        done = 1i32;
                    } else {
                        current =
                            catsdscatlen(current, p as (*const ::std::os::raw::c_void), 1usize);
                    }
                }
                if *p == 0 {
                    continue;
                }
                p = p.offset(1isize);
            }
            vector = realloc(
                vector as (*mut ::std::os::raw::c_void),
                ((*argc + 1i32) as (usize)).wrapping_mul(::std::mem::size_of::<*mut u8>()),
            ) as (*mut *mut u8);
            *vector.offset(*argc as (isize)) = current;
            *argc = *argc + 1;
            current = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
    }
    if _currentBlock == 3 {
        if vector == 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) {
            vector = malloc(::std::mem::size_of::<*mut ::std::os::raw::c_void>()) as (*mut *mut u8);
        }
        vector
    } else {
        'loop34: loop {
            if {
                let _old = *argc;
                *argc = *argc - 1;
                _old
            } == 0
            {
                break;
            }
            catsdsfree(*vector.offset(*argc as (isize)));
        }
        free(vector as (*mut ::std::os::raw::c_void));
        if !current.is_null() {
            catsdsfree(current);
        }
        *argc = 0i32;
        0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8)
    }
}

#[no_mangle]
pub unsafe extern "C" fn catsdsmapchars(
    mut s: *mut u8,
    mut from: *const u8,
    mut to: *const u8,
    mut setlen: usize,
) -> *mut u8 {
    let mut _currentBlock;
    let mut j: usize;
    let mut i: usize;
    let mut l: usize = catsdslen(s);
    j = 0usize;
    'loop1: loop {
        if !(j < l) {
            break;
        }
        i = 0usize;
        'loop4: loop {
            if !(i < setlen) {
                _currentBlock = 8;
                break;
            }
            if *s.offset(j as (isize)) as (i32) == *from.offset(i as (isize)) as (i32) {
                _currentBlock = 7;
                break;
            }
            i = i.wrapping_add(1usize);
        }
        if _currentBlock == 7 {
            *s.offset(j as (isize)) = *to.offset(i as (isize));
        }
        j = j.wrapping_add(1usize);
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn catsdsjoin(
    mut argv: *mut *mut u8,
    mut argc: i32,
    mut sep: *mut u8,
) -> *mut u8 {
    let mut join: *mut u8 = catsdsempty();
    let mut j: i32;
    j = 0i32;
    'loop1: loop {
        if !(j < argc) {
            break;
        }
        join = catsdscat(join, *argv.offset(j as (isize)) as (*const u8));
        if j != argc - 1i32 {
            join = catsdscat(join, sep as (*const u8));
        }
        j = j + 1;
    }
    join
}
