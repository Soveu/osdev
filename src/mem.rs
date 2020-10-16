#[inline(never)]
pub unsafe fn rep_movsb(src: *const u8, dst: *mut u8, count: usize) {
    asm!(
        "rep movsb BYTE PTR [rdi], BYTE PTR [rsi]",
        inout("rcx") count => _,
        inout("rsi") src => _,
        inout("rdi") dst => _,
        options(nostack, preserves_flags)
    );
}

#[inline(always)]
pub unsafe fn rep_stosb(dst: *mut u8, val: u8, count: usize) {
    asm!(
        "rep stosb BYTE PTR [rdi]",
        inout("rcx") count=> _,
        inout("rdi") dst => _,
        in("al") val,
        options(nostack, preserves_flags)
    );
}

#[inline(always)]
pub unsafe fn repe_cmpsb(
    mut a: *const u8,
    mut b: *const u8,
    mut n: usize,
) -> (*const u8, *const u8, usize) {
    asm!(
        "rep cmpsb BYTE PTR [rdi], BYTE PTR [rsi]",
        inout("rcx") n => n,
        inout("rsi") a => a,
        inout("rdi") b => b,
        options(nostack, preserves_flags)
    );

    return (a, b, n);
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    rep_movsb(src, dst, n);
    return dst;
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, count: usize) -> *mut u8 {
    if dst == src as *mut u8 {
        return dst;
    }

    /* https://github.com/rust-lang/compiler-builtins/pull/365 */
    //let dst_end = dst.offset(count as isize);
    let src_end = src.offset(count as isize);

    if src >= dst as *const u8 || src_end <= dst as *const u8 {
        return memcpy(dst, src, count);
    }

    /* Copy backwards */
    asm!(
        "std",
        "rep movsb BYTE PTR [rdi], BYTE PTR [rsi]",
        "cld",
        inout("rcx") count => _,
        inout("rsi") src => _,
        inout("rdi") dst => _,
        options(nostack, preserves_flags)
    );

    return dst;
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    rep_stosb(s, c as u8, n);
    return s;
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }

    let (a, b, _) = repe_cmpsb(a, b, n);
    let res: u8 = *a.offset(-1) - *b.offset(-1);
    return res as i8 as i32;
}
