extern {
    static mut _DefaultRuneLocale : Struct1;
    fn __maskrune(arg1 : i32, arg2 : usize) -> i32;
    fn __swbuf(arg1 : i32, arg2 : *mut __sFILE) -> i32;
    fn __tolower(arg1 : i32) -> i32;
    fn __toupper(arg1 : i32) -> i32;
    fn calloc(
        __count : usize, __size : usize
    ) -> *mut ::std::os::raw::c_void;
    fn catsdsavail(s : *mut u8) -> usize;
    fn catsdscatprintf(s : *mut u8, fmt : *const u8, ...) -> *mut u8;
    fn catsdslen(s : *mut u8) -> usize;
    fn free(arg1 : *mut ::std::os::raw::c_void);
    fn malloc(__size : usize) -> *mut ::std::os::raw::c_void;
    fn memcmp(
        __s1 : *const ::std::os::raw::c_void,
        __s2 : *const ::std::os::raw::c_void,
        __n : usize
    ) -> i32;
    fn memcpy(
        __dst : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __b : *mut ::std::os::raw::c_void, __c : i32, __len : usize
    ) -> *mut ::std::os::raw::c_void;
    fn realloc(
        __ptr : *mut ::std::os::raw::c_void, __size : usize
    ) -> *mut ::std::os::raw::c_void;
    fn strchr(__s : *const u8, __c : i32) -> *mut u8;
    fn strlen(__s : *const u8) -> usize;
}

enum __sFILEX {
}

#[derive(Copy)]
#[repr(C)]
pub struct __sbuf {
    pub _base : *mut u8,
    pub _size : i32,
}

impl Clone for __sbuf {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct __sFILE {
    pub _p : *mut u8,
    pub _r : i32,
    pub _w : i32,
    pub _flags : i16,
    pub _file : i16,
    pub _bf : __sbuf,
    pub _lbfsize : i32,
    pub _cookie : *mut ::std::os::raw::c_void,
    pub _close : unsafe extern fn(*mut ::std::os::raw::c_void) -> i32,
    pub _read : unsafe extern fn(*mut ::std::os::raw::c_void, *mut u8, i32) -> i32,
    pub _seek : unsafe extern fn(*mut ::std::os::raw::c_void, isize, i32) -> isize,
    pub _write : unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, i32) -> i32,
    pub _ub : __sbuf,
    pub _extra : *mut __sFILEX,
    pub _ur : i32,
    pub _ubuf : [u8; 3],
    pub _nbuf : [u8; 1],
    pub _lb : __sbuf,
    pub _blksize : i32,
    pub _offset : isize,
}

impl Clone for __sFILE {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __sputc(
    mut _c : i32, mut _p : *mut __sFILE
) -> i32 {
    if {
           (*_p)._w = (*_p)._w - 1;
           (*_p)._w
       } >= 0i32 || (*_p)._w >= (*_p)._lbfsize && (_c as (u8) as (i32) != b'\n' as (i32)) {
        ({
             let _rhs = _c;
             let _lhs
                 = &mut *{
                             let _old = (*_p)._p;
                             (*_p)._p = (*_p)._p.offset(1isize);
                             _old
                         };
             *_lhs = _rhs as (u8);
             *_lhs
         }) as (i32)
    } else {
        __swbuf(_c,_p)
    }
}

#[no_mangle]
pub unsafe extern fn isascii(mut _c : i32) -> i32 {
    (_c & !0x7fi32 == 0i32) as (i32)
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub __min : i32,
    pub __max : i32,
    pub __map : i32,
    pub __types : *mut u32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub __nranges : i32,
    pub __ranges : *mut Struct3,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub __name : [u8; 14],
    pub __mask : u32,
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub __magic : [u8; 8],
    pub __encoding : [u8; 32],
    pub __sgetrune : unsafe extern fn(*const u8, usize, *mut *const u8) -> i32,
    pub __sputrune : unsafe extern fn(i32, *mut u8, usize, *mut *mut u8) -> i32,
    pub __invalid_rune : i32,
    pub __runetype : [u32; 256],
    pub __maplower : [i32; 256],
    pub __mapupper : [i32; 256],
    pub __runetype_ext : Struct2,
    pub __maplower_ext : Struct2,
    pub __mapupper_ext : Struct2,
    pub __variable : *mut ::std::os::raw::c_void,
    pub __variable_len : i32,
    pub __ncharclasses : i32,
    pub __charclasses : *mut Struct4,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __istype(mut _c : i32, mut _f : usize) -> i32 {
    if isascii(_c) != 0 {
        !(_DefaultRuneLocale.__runetype[
              _c as (usize)
          ] as (usize) & _f == 0) as (i32)
    } else {
        !(__maskrune(_c,_f) == 0) as (i32)
    }
}

#[no_mangle]
pub unsafe extern fn __isctype(mut _c : i32, mut _f : usize) -> i32 {
    if _c < 0i32 || _c >= 256i32 {
        0i32
    } else {
        !(_DefaultRuneLocale.__runetype[
              _c as (usize)
          ] as (usize) & _f == 0) as (i32)
    }
}

#[no_mangle]
pub unsafe extern fn __wcwidth(mut _c : i32) -> i32 {
    let mut _x : u32;
    if _c == 0i32 {
        0i32
    } else {
        _x = __maskrune(_c,0xe0000000usize | 0x40000usize) as (u32);
        (if _x as (usize) & 0xe0000000usize != 0usize {
             ((_x as (usize) & 0xe0000000usize) >> 30i32) as (i32)
         } else if _x as (usize) & 0x40000usize != 0usize {
             1i32
         } else {
             -1i32
         })
    }
}

#[no_mangle]
pub unsafe extern fn isalnum(mut _c : i32) -> i32 {
    __istype(_c,(0x100isize | 0x400isize) as (usize))
}

#[no_mangle]
pub unsafe extern fn isalpha(mut _c : i32) -> i32 {
    __istype(_c,0x100usize)
}

#[no_mangle]
pub unsafe extern fn isblank(mut _c : i32) -> i32 {
    __istype(_c,0x20000usize)
}

#[no_mangle]
pub unsafe extern fn iscntrl(mut _c : i32) -> i32 {
    __istype(_c,0x200usize)
}

#[no_mangle]
pub unsafe extern fn isdigit(mut _c : i32) -> i32 {
    __isctype(_c,0x400usize)
}

#[no_mangle]
pub unsafe extern fn isgraph(mut _c : i32) -> i32 {
    __istype(_c,0x800usize)
}

#[no_mangle]
pub unsafe extern fn islower(mut _c : i32) -> i32 {
    __istype(_c,0x1000usize)
}

#[no_mangle]
pub unsafe extern fn isprint(mut _c : i32) -> i32 {
    __istype(_c,0x40000usize)
}

#[no_mangle]
pub unsafe extern fn ispunct(mut _c : i32) -> i32 {
    __istype(_c,0x2000usize)
}

#[no_mangle]
pub unsafe extern fn isspace(mut _c : i32) -> i32 {
    __istype(_c,0x4000usize)
}

#[no_mangle]
pub unsafe extern fn isupper(mut _c : i32) -> i32 {
    __istype(_c,0x8000usize)
}

#[no_mangle]
pub unsafe extern fn isxdigit(mut _c : i32) -> i32 {
    __isctype(_c,0x10000usize)
}

#[no_mangle]
pub unsafe extern fn toascii(mut _c : i32) -> i32 { _c & 0x7fi32 }

#[no_mangle]
pub unsafe extern fn tolower(mut _c : i32) -> i32 { __tolower(_c) }

#[no_mangle]
pub unsafe extern fn toupper(mut _c : i32) -> i32 { __toupper(_c) }

#[no_mangle]
pub unsafe extern fn digittoint(mut _c : i32) -> i32 {
    __maskrune(_c,0xfusize)
}

#[no_mangle]
pub unsafe extern fn ishexnumber(mut _c : i32) -> i32 {
    __istype(_c,0x10000usize)
}

#[no_mangle]
pub unsafe extern fn isideogram(mut _c : i32) -> i32 {
    __istype(_c,0x80000usize)
}

#[no_mangle]
pub unsafe extern fn isnumber(mut _c : i32) -> i32 {
    __istype(_c,0x400usize)
}

#[no_mangle]
pub unsafe extern fn isphonogram(mut _c : i32) -> i32 {
    __istype(_c,0x200000usize)
}

#[no_mangle]
pub unsafe extern fn isrune(mut _c : i32) -> i32 {
    __istype(_c,0xfffffff0usize)
}

#[no_mangle]
pub unsafe extern fn isspecial(mut _c : i32) -> i32 {
    __istype(_c,0x100000usize)
}

#[derive(Copy)]
#[repr(C)]
pub struct _sdshdr {
    pub len : u32,
    pub free : u32,
}

impl Clone for _sdshdr {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn catsdsnewlen(
    mut init : *const ::std::os::raw::c_void, mut initlen : usize
) -> *mut u8 {
    let mut sh : *mut _sdshdr;
    if !init.is_null() {
        sh = malloc(
                 ::std::mem::size_of::<_sdshdr>().wrapping_add(
                     initlen
                 ).wrapping_add(
                     1usize
                 )
             ) as (*mut _sdshdr);
    } else {
        sh = calloc(
                 ::std::mem::size_of::<_sdshdr>().wrapping_add(
                     initlen
                 ).wrapping_add(
                     1usize
                 ),
                 1usize
             ) as (*mut _sdshdr);
    }
    if sh == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _sdshdr) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*sh).len = initlen as (u32);
        (*sh).free = 0u32;
        0i32 as (*mut u8)
    }
}

#[no_mangle]
pub unsafe extern fn catsdsnewEmpty(
    mut preAlloclen : usize
) -> *mut u8 {
    let mut sh : *mut _sdshdr;
    sh = malloc(
             ::std::mem::size_of::<_sdshdr>().wrapping_add(
                 preAlloclen
             ).wrapping_add(
                 1usize
             )
         ) as (*mut _sdshdr);
    if sh == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _sdshdr) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*sh).len = 0u32;
        (*sh).free = preAlloclen as (u32);
        0i32 as (*mut u8)
    }
}

#[no_mangle]
pub unsafe extern fn catsdsempty() -> *mut u8 {
    catsdsnewlen(
        (*b"\0").as_ptr() as (*const ::std::os::raw::c_void),
        0usize
    )
}

#[no_mangle]
pub unsafe extern fn catsdsnew(mut init : *const u8) -> *mut u8 {
    let mut initlen
        : usize
        = if init == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
              0usize
          } else {
              strlen(init)
          };
    catsdsnewlen(init as (*const ::std::os::raw::c_void),initlen)
}

#[no_mangle]
pub unsafe extern fn catsdsdup(s : *mut u8) -> *mut u8 {
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        catsdsnewlen(s as (*const ::std::os::raw::c_void),catsdslen(s))
    }
}

#[no_mangle]
pub unsafe extern fn catsdsfree(mut s : *mut u8) {
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
    } else {
        free(
            s.offset(
                -(::std::mem::size_of::<_sdshdr>() as (isize))
            ) as (*mut ::std::os::raw::c_void)
        );
    }
}

#[no_mangle]
pub unsafe extern fn catsdsupdatelen(mut s : *mut u8) {
    let mut sh
        : *mut _sdshdr
        = s.offset(
              -(::std::mem::size_of::<_sdshdr>() as (isize))
          ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    let mut reallen : i32 = strlen(s as (*const u8)) as (i32);
    (*sh).free = (*sh).free.wrapping_add(
                     (*sh).len.wrapping_sub(reallen as (u32))
                 );
    (*sh).len = reallen as (u32);
}

#[no_mangle]
pub unsafe extern fn catsdsclear(mut s : *mut u8) {
    let mut sh
        : *mut _sdshdr
        = s.offset(
              -(::std::mem::size_of::<_sdshdr>() as (isize))
          ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    (*sh).free = (*sh).free.wrapping_add((*sh).len);
    (*sh).len = 0u32;
}

#[no_mangle]
pub unsafe extern fn catsdsMakeRoomFor(
    mut s : *mut u8, mut addlen : usize
) -> *mut u8 {
    let mut sh : *mut _sdshdr;
    let mut newsh : *mut _sdshdr;
    let mut free : usize = catsdsavail(s);
    let mut len : usize;
    let mut newlen : usize;
    if free >= addlen {
        s
    } else {
        len = catsdslen(s);
        sh = s.offset(
                 -(::std::mem::size_of::<_sdshdr>() as (isize))
             ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
        newlen = len.wrapping_add(addlen);
        if newlen < (1024i32 * 1024i32) as (usize) {
            newlen = newlen.wrapping_mul(2usize);
        } else {
            newlen = newlen.wrapping_add((1024i32 * 1024i32) as (usize));
        }
        newsh = realloc(
                    sh as (*mut ::std::os::raw::c_void),
                    ::std::mem::size_of::<_sdshdr>().wrapping_add(newlen).wrapping_add(
                        1usize
                    )
                ) as (*mut _sdshdr);
        (if newsh == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _sdshdr) {
             0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
         } else {
             (*newsh).free = newlen.wrapping_sub(len) as (u32);
             0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
         })
    }
}

#[no_mangle]
pub unsafe extern fn catsdsRemoveFreeSpace(
    mut s : *mut u8
) -> *mut u8 {
    let mut sh : *mut _sdshdr;
    sh = s.offset(
             -(::std::mem::size_of::<_sdshdr>() as (isize))
         ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    sh = realloc(
             sh as (*mut ::std::os::raw::c_void),
             ::std::mem::size_of::<_sdshdr>().wrapping_add(
                 (*sh).len as (usize)
             ).wrapping_add(
                 1usize
             )
         ) as (*mut _sdshdr);
    (*sh).free = 0u32;
    0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
}

#[no_mangle]
pub unsafe extern fn catsdsAllocSize(mut s : *mut u8) -> usize {
    let mut sh
        : *mut _sdshdr
        = s.offset(
              -(::std::mem::size_of::<_sdshdr>() as (isize))
          ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    ::std::mem::size_of::<_sdshdr>().wrapping_add(
        (*sh).len as (usize)
    ).wrapping_add(
        (*sh).free as (usize)
    ).wrapping_add(
        1usize
    )
}

#[no_mangle]
pub unsafe extern fn catsdsIncrLen(mut s : *mut u8, mut incr : i32) {
    let mut sh
        : *mut _sdshdr
        = s.offset(
              -(::std::mem::size_of::<_sdshdr>() as (isize))
          ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    (*sh).len = (*sh).len.wrapping_add(incr as (u32));
    (*sh).free = (*sh).free.wrapping_sub(incr as (u32));
    *s.offset((*sh).len as (isize)) = b'\0';
}

#[no_mangle]
pub unsafe extern fn catsdsgrowzero(
    mut s : *mut u8, mut len : usize
) -> *mut u8 {
    let mut sh
        : *mut _sdshdr
        = s.offset(
              -(::std::mem::size_of::<_sdshdr>() as (isize))
          ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    let mut totlen : usize;
    let mut curlen : usize = (*sh).len as (usize);
    if len <= curlen {
        s
    } else {
        s = catsdsMakeRoomFor(s,len.wrapping_sub(curlen));
        (if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
             0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
         } else {
             sh = s.offset(
                      -(::std::mem::size_of::<_sdshdr>() as (isize))
                  ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
             memset(
                 s.offset(curlen as (isize)) as (*mut ::std::os::raw::c_void),
                 0i32,
                 len.wrapping_sub(curlen).wrapping_add(1usize)
             );
             totlen = (*sh).len.wrapping_add((*sh).free) as (usize);
             (*sh).len = len as (u32);
             (*sh).free = totlen.wrapping_sub((*sh).len as (usize)) as (u32);
             s
         })
    }
}

#[no_mangle]
pub unsafe extern fn catsdscatlen(
    mut s : *mut u8,
    mut t : *const ::std::os::raw::c_void,
    mut len : usize
) -> *mut u8 {
    let mut sh : *mut _sdshdr;
    let mut curlen : usize = catsdslen(s);
    s = catsdsMakeRoomFor(s,len);
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        sh = s.offset(
                 -(::std::mem::size_of::<_sdshdr>() as (isize))
             ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
        memcpy(
            s.offset(curlen as (isize)) as (*mut ::std::os::raw::c_void),
            t,
            len
        );
        (*sh).len = curlen.wrapping_add(len) as (u32);
        (*sh).free = ((*sh).free as (usize)).wrapping_sub(len) as (u32);
        *s.offset(curlen.wrapping_add(len) as (isize)) = b'\0';
        s
    }
}

#[no_mangle]
pub unsafe extern fn catsdscatchar(
    mut s : *mut u8, mut c : u8
) -> *mut u8 {
    let mut sh : *mut _sdshdr;
    let mut curlen : usize = catsdslen(s);
    s = catsdsMakeRoomFor(s,1usize);
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        sh = s.offset(
                 -(::std::mem::size_of::<_sdshdr>() as (isize))
             ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
        *s.offset(curlen as (isize)) = c;
        *s.offset(curlen.wrapping_add(1usize) as (isize)) = b'\0';
        (*sh).len = (*sh).len.wrapping_add(1u32);
        (*sh).free = (*sh).free.wrapping_sub(1u32);
        s
    }
}

#[no_mangle]
pub unsafe extern fn catsdscat(
    mut s : *mut u8, mut t : *const u8
) -> *mut u8 {
    if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) || t == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        s
    } else {
        catsdscatlen(s,t as (*const ::std::os::raw::c_void),strlen(t))
    }
}

#[no_mangle]
pub unsafe extern fn catsdscatsds(
    mut s : *mut u8, t : *mut u8
) -> *mut u8 {
    catsdscatlen(s,t as (*const ::std::os::raw::c_void),catsdslen(t))
}

#[no_mangle]
pub unsafe extern fn catsdscpylen(
    mut s : *mut u8, mut t : *const u8, mut len : usize
) -> *mut u8 {
    let mut sh
        : *mut _sdshdr
        = s.offset(
              -(::std::mem::size_of::<_sdshdr>() as (isize))
          ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    let mut totlen
        : usize
        = (*sh).free.wrapping_add((*sh).len) as (usize);
    if totlen < len {
        s = catsdsMakeRoomFor(s,len.wrapping_sub((*sh).len as (usize)));
        if s == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            return 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        } else {
            sh = s.offset(
                     -(::std::mem::size_of::<_sdshdr>() as (isize))
                 ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
            totlen = (*sh).free.wrapping_add((*sh).len) as (usize);
        }
    }
    memcpy(
        s as (*mut ::std::os::raw::c_void),
        t as (*const ::std::os::raw::c_void),
        len
    );
    *s.offset(len as (isize)) = b'\0';
    (*sh).len = len as (u32);
    (*sh).free = totlen.wrapping_sub(len) as (u32);
    s
}

#[no_mangle]
pub unsafe extern fn catsdscpy(
    mut s : *mut u8, mut t : *const u8
) -> *mut u8 {
    catsdscpylen(s,t,strlen(t))
}

#[no_mangle]
pub unsafe extern fn sdsll2str(
    mut s : *mut u8, mut value : isize
) -> i32 {
    let mut p : *mut u8;
    let mut aux : u8;
    let mut v : usize;
    let mut l : usize;
    v = if value < 0isize { -value } else { value } as (usize);
    p = s;
    'loop1: loop {
        *{
             let _old = p;
             p = p.offset(1isize);
             _old
         } = (b'0' as (usize)).wrapping_add(
                 v.wrapping_rem(10usize)
             ) as (u8);
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
    l = ((p as (isize)).wrapping_sub(
             s as (isize)
         ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
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
pub unsafe extern fn sdsull2str(
    mut s : *mut u8, mut v : usize
) -> i32 {
    let mut p : *mut u8;
    let mut aux : u8;
    let mut l : usize;
    p = s;
    'loop1: loop {
        *{
             let _old = p;
             p = p.offset(1isize);
             _old
         } = (b'0' as (usize)).wrapping_add(
                 v.wrapping_rem(10usize)
             ) as (u8);
        v = v.wrapping_div(10usize);
        if v == 0 {
            break;
        }
    }
    l = ((p as (isize)).wrapping_sub(
             s as (isize)
         ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
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
pub unsafe extern fn catsdsfromlonglong(mut value : isize) -> *mut u8 {
    let mut buf : [u8; 21];
    let mut len : i32 = sdsll2str(buf.as_mut_ptr(),value);
    catsdsnewlen(
        buf.as_mut_ptr() as (*const ::std::os::raw::c_void),
        len as (usize)
    )
}

#[no_mangle]
pub unsafe extern fn catsdstrim(
    mut s : *mut u8, mut cset : *const u8
) -> *mut u8 {
    let mut sh
        : *mut _sdshdr
        = s.offset(
              -(::std::mem::size_of::<_sdshdr>() as (isize))
          ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    let mut start : *mut u8;
    let mut end : *mut u8;
    let mut sp : *mut u8;
    let mut ep : *mut u8;
    let mut len : usize;
    sp = {
             start = s;
             start
         };
    ep = {
             end = s.offset(catsdslen(s) as (isize)).offset(-1isize);
             end
         };
    'loop1: loop {
        if !(sp <= end && !strchr(cset,*sp as (i32)).is_null()) {
            break;
        }
        sp = sp.offset(1isize);
    }
    'loop2: loop {
        if !(ep > start && !strchr(cset,*ep as (i32)).is_null()) {
            break;
        }
        ep = ep.offset(-1isize);
    }
    len = if sp > ep {
              0isize
          } else {
              (ep as (isize)).wrapping_sub(
                  sp as (isize)
              ) / ::std::mem::size_of::<u8>() as (isize) + 1isize
          } as (usize);
    (*sh).free = ((*sh).free as (usize)).wrapping_add(
                     ((*sh).len as (usize)).wrapping_sub(len)
                 ) as (u32);
    (*sh).len = len as (u32);
    s
}

#[no_mangle]
pub unsafe extern fn catsdsrange(
    mut s : *mut u8, mut start : i32, mut end : i32
) {
    let mut sh
        : *mut _sdshdr
        = s.offset(
              -(::std::mem::size_of::<_sdshdr>() as (isize))
          ) as (*mut ::std::os::raw::c_void) as (*mut _sdshdr);
    let mut newlen : usize;
    let mut len : usize = catsdslen(s);
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
        (*sh).free = ((*sh).free as (usize)).wrapping_add(
                         ((*sh).len as (usize)).wrapping_sub(newlen)
                     ) as (u32);
        (*sh).len = newlen as (u32);
    }
}

#[no_mangle]
pub unsafe extern fn catsdstolower(mut s : *mut u8) {
    let mut len : i32 = catsdslen(s) as (i32);
    let mut j : i32;
    j = 0i32;
    'loop1: loop {
        if !(j < len) {
            break;
        }
        *s.offset(j as (isize)) = tolower(
                                      *s.offset(j as (isize)) as (i32)
                                  ) as (u8);
        j = j + 1;
    }
}

#[no_mangle]
pub unsafe extern fn catsdstoupper(mut s : *mut u8) {
    let mut len : i32 = catsdslen(s) as (i32);
    let mut j : i32;
    j = 0i32;
    'loop1: loop {
        if !(j < len) {
            break;
        }
        *s.offset(j as (isize)) = toupper(
                                      *s.offset(j as (isize)) as (i32)
                                  ) as (u8);
        j = j + 1;
    }
}

#[no_mangle]
pub unsafe extern fn catsdscmp(s1 : *mut u8, s2 : *mut u8) -> i32 {
    let mut l1 : usize;
    let mut l2 : usize;
    let mut minlen : usize;
    let mut cmp : i32;
    l1 = catsdslen(s1);
    l2 = catsdslen(s2);
    minlen = if l1 < l2 { l1 } else { l2 };
    cmp = memcmp(
              s1 as (*const ::std::os::raw::c_void),
              s2 as (*const ::std::os::raw::c_void),
              minlen
          );
    if cmp == 0i32 { l1.wrapping_sub(l2) as (i32) } else { cmp }
}

#[no_mangle]
pub unsafe extern fn catsdssplitlen(
    mut s : *const u8,
    mut len : i32,
    mut sep : *const u8,
    mut seplen : i32,
    mut count : *mut i32
) -> *mut *mut u8 {
    let mut _currentBlock;
    let mut elements : i32 = 0i32;
    let mut slots : i32 = 5i32;
    let mut start : i32 = 0i32;
    let mut j : i32;
    let mut tokens : *mut *mut u8;
    if seplen < 1i32 || len < 0i32 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8)
    } else {
        tokens = malloc(
                     ::std::mem::size_of::<*mut u8>().wrapping_mul(slots as (usize))
                 ) as (*mut *mut u8);
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
                     let mut newtokens : *mut *mut u8;
                     slots = slots * 2i32;
                     newtokens = realloc(
                                     tokens as (*mut ::std::os::raw::c_void),
                                     ::std::mem::size_of::<*mut u8>().wrapping_mul(slots as (usize))
                                 ) as (*mut *mut u8);
                     if newtokens == 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) {
                         _currentBlock = 14;
                         break;
                     }
                     tokens = newtokens;
                 }
                 if seplen == 1i32 && (*s.offset(
                                            j as (isize)
                                        ) as (i32) == *sep.offset(0isize) as (i32)) || memcmp(
                                                                                           s.offset(
                                                                                               j as (isize)
                                                                                           ) as (*const ::std::os::raw::c_void),
                                                                                           sep as (*const ::std::os::raw::c_void),
                                                                                           seplen as (usize)
                                                                                       ) == 0i32 {
                     *tokens.offset(elements as (isize)) = catsdsnewlen(
                                                               s.offset(
                                                                   start as (isize)
                                                               ) as (*const ::std::os::raw::c_void),
                                                               (j - start) as (usize)
                                                           );
                     if *tokens.offset(
                             elements as (isize)
                         ) == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
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
                                                           s.offset(
                                                               start as (isize)
                                                           ) as (*const ::std::os::raw::c_void),
                                                           (len - start) as (usize)
                                                       );
                 if !(*tokens.offset(
                           elements as (isize)
                       ) == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
                     elements = elements + 1;
                     *count = elements;
                     return tokens;
                 }
             }
             let mut i : i32;
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
pub unsafe extern fn catsdsfreesplitres(
    mut tokens : *mut *mut u8, mut count : i32
) { if tokens.is_null() {
    } else {
        'loop1: loop {
            if {
                   let _old = count;
                   count = count - 1;
                   _old
               } == 0 {
                break;
            }
            catsdsfree(*tokens.offset(count as (isize)));
        }
        free(tokens as (*mut ::std::os::raw::c_void));
    }
}

#[no_mangle]
pub unsafe extern fn catsdscatrepr(
    mut s : *mut u8, mut p : *const u8, mut len : usize
) -> *mut u8 {
    s = catsdscatlen(
            s,
            (*b"\"\0").as_ptr() as (*const ::std::os::raw::c_void),
            1usize
        );
    'loop1: loop {
        if {
               let _old = len;
               len = len.wrapping_sub(1usize);
               _old
           } == 0 {
            break;
        }
        let switch5 = *p;
        if switch5 as (i32) == b'\x08' as (i32) {
            s = catsdscatlen(
                    s,
                    (*b"\\b\0").as_ptr() as (*const ::std::os::raw::c_void),
                    2usize
                );
        } else if switch5 as (i32) == b'\x07' as (i32) {
            s = catsdscatlen(
                    s,
                    (*b"\\a\0").as_ptr() as (*const ::std::os::raw::c_void),
                    2usize
                );
        } else if switch5 as (i32) == b'\t' as (i32) {
            s = catsdscatlen(
                    s,
                    (*b"\\t\0").as_ptr() as (*const ::std::os::raw::c_void),
                    2usize
                );
        } else if switch5 as (i32) == b'\r' as (i32) {
            s = catsdscatlen(
                    s,
                    (*b"\\r\0").as_ptr() as (*const ::std::os::raw::c_void),
                    2usize
                );
        } else if switch5 as (i32) == b'\n' as (i32) {
            s = catsdscatlen(
                    s,
                    (*b"\\n\0").as_ptr() as (*const ::std::os::raw::c_void),
                    2usize
                );
        } else if switch5 as (i32) == b'\"' as (i32) || switch5 as (i32) == b'\\' as (i32) {
            s = catsdscatprintf(s,(*b"\\%c\0").as_ptr(),*p as (i32));
        } else if isprint(*p as (i32)) != 0 {
            s = catsdscatprintf(s,(*b"%c\0").as_ptr(),*p as (i32));
        } else {
            s = catsdscatprintf(s,(*b"\\x%02x\0").as_ptr(),*p as (i32));
        }
        p = p.offset(1isize);
    }
    catsdscatlen(
        s,
        (*b"\"\0").as_ptr() as (*const ::std::os::raw::c_void),
        1usize
    )
}

#[no_mangle]
pub unsafe extern fn is_hex_digit(mut c : u8) -> i32 {
    (c as (i32) >= b'0' as (i32) && (c as (i32) <= b'9' as (i32)) || c as (i32) >= b'a' as (i32) && (c as (i32) <= b'f' as (i32)) || c as (i32) >= b'A' as (i32) && (c as (i32) <= b'F' as (i32))) as (i32)
}

#[no_mangle]
pub unsafe extern fn hex_digit_to_int(mut c : u8) -> i32 {
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
pub unsafe extern fn catsdssplitargs(
    mut line : *const u8, mut argc : *mut i32
) -> *mut *mut u8 {
    let mut _currentBlock;
    let mut p : *const u8 = line;
    let mut current
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut vector
        : *mut *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8);
    *argc = 0i32;
    'loop1: loop {
        if *p != 0 && (isspace(*p as (i32)) != 0) {
            p = p.offset(1isize);
        } else {
            if *p == 0 {
                _currentBlock = 3;
                break;
            }
            let mut inq : i32 = 0i32;
            let mut insq : i32 = 0i32;
            let mut done : i32 = 0i32;
            if current == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                current = catsdsempty();
            }
            'loop8: loop {
                if !(done == 0) {
                    break;
                }
                if inq != 0 {
                    if *p as (i32) == b'\\' as (i32) && (*p.offset(
                                                              1isize
                                                          ) as (i32) == b'x' as (i32)) && (is_hex_digit(
                                                                                               *p.offset(
                                                                                                    2isize
                                                                                                )
                                                                                           ) != 0) && (is_hex_digit(
                                                                                                           *p.offset(
                                                                                                                3isize
                                                                                                            )
                                                                                                       ) != 0) {
                        let mut byte : u8;
                        byte = (hex_digit_to_int(
                                    *p.offset(2isize)
                                ) * 16i32 + hex_digit_to_int(*p.offset(3isize))) as (u8);
                        current = catsdscatlen(
                                      current,
                                      &mut byte as (*mut u8) as (*const ::std::os::raw::c_void),
                                      1usize
                                  );
                        p = p.offset(3isize);
                    } else if *p as (i32) == b'\\' as (i32) && (*p.offset(
                                                                     1isize
                                                                 ) != 0) {
                        let mut c : u8;
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
                                      1usize
                                  );
                    } else if *p as (i32) == b'\"' as (i32) {
                        if *p.offset(1isize) != 0 && (isspace(
                                                          *p.offset(1isize) as (i32)
                                                      ) == 0) {
                            _currentBlock = 34;
                            break 'loop1;
                        }
                        done = 1i32;
                    } else {
                        if *p == 0 {
                            _currentBlock = 34;
                            break 'loop1;
                        }
                        current = catsdscatlen(
                                      current,
                                      p as (*const ::std::os::raw::c_void),
                                      1usize
                                  );
                    }
                } else if insq != 0 {
                    if *p as (i32) == b'\\' as (i32) && (*p.offset(
                                                              1isize
                                                          ) as (i32) == b'\'' as (i32)) {
                        p = p.offset(1isize);
                        current = catsdscatlen(
                                      current,
                                      (*b"\'\0").as_ptr() as (*const ::std::os::raw::c_void),
                                      1usize
                                  );
                    } else if *p as (i32) == b'\'' as (i32) {
                        if *p.offset(1isize) != 0 && (isspace(
                                                          *p.offset(1isize) as (i32)
                                                      ) == 0) {
                            _currentBlock = 34;
                            break 'loop1;
                        }
                        done = 1i32;
                    } else {
                        if *p == 0 {
                            _currentBlock = 34;
                            break 'loop1;
                        }
                        current = catsdscatlen(
                                      current,
                                      p as (*const ::std::os::raw::c_void),
                                      1usize
                                  );
                    }
                } else {
                    let switch6 = *p;
                    if switch6 as (i32) == b'\'' as (i32) {
                        insq = 1i32;
                    } else if switch6 as (i32) == b'\"' as (i32) {
                        inq = 1i32;
                    } else if switch6 as (i32) == b'\0' as (i32) || switch6 as (i32) == b'\t' as (i32) || switch6 as (i32) == b'\r' as (i32) || switch6 as (i32) == b'\n' as (i32) || switch6 as (i32) == b' ' as (i32) {
                        done = 1i32;
                    } else {
                        current = catsdscatlen(
                                      current,
                                      p as (*const ::std::os::raw::c_void),
                                      1usize
                                  );
                    }
                }
                if *p == 0 {
                    continue;
                }
                p = p.offset(1isize);
            }
            vector = realloc(
                         vector as (*mut ::std::os::raw::c_void),
                         ((*argc + 1i32) as (usize)).wrapping_mul(
                             ::std::mem::size_of::<*mut u8>()
                         )
                     ) as (*mut *mut u8);
            *vector.offset(*argc as (isize)) = current;
            *argc = *argc + 1;
            current = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
    }
    if _currentBlock == 3 {
        if vector == 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) {
            vector = malloc(
                         ::std::mem::size_of::<*mut ::std::os::raw::c_void>()
                     ) as (*mut *mut u8);
        }
        vector
    } else {
        'loop34: loop {
            if {
                   let _old = *argc;
                   *argc = *argc - 1;
                   _old
               } == 0 {
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
pub unsafe extern fn catsdsmapchars(
    mut s : *mut u8,
    mut from : *const u8,
    mut to : *const u8,
    mut setlen : usize
) -> *mut u8 {
    let mut _currentBlock;
    let mut j : usize;
    let mut i : usize;
    let mut l : usize = catsdslen(s);
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
            if *s.offset(j as (isize)) as (i32) == *from.offset(
                                                        i as (isize)
                                                    ) as (i32) {
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
pub unsafe extern fn catsdsjoin(
    mut argv : *mut *mut u8, mut argc : i32, mut sep : *mut u8
) -> *mut u8 {
    let mut join : *mut u8 = catsdsempty();
    let mut j : i32;
    j = 0i32;
    'loop1: loop {
        if !(j < argc) {
            break;
        }
        join = catsdscat(join,*argv.offset(j as (isize)) as (*const u8));
        if j != argc - 1i32 {
            join = catsdscat(join,sep as (*const u8));
        }
        j = j + 1;
    }
    join
}
