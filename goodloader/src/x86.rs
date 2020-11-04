#[inline(always)]
pub fn halt() {
    /* SAFETY: we are ring 0 */
    unsafe {
        asm!("hlt", options(nostack));
    }
}
#[inline(always)]
pub fn disable_interrupts() {
    /* SAFETY: we are ring 0 */
    unsafe {
        asm!("cli", options(nostack));
    }
}
#[inline(always)]
pub fn nop() {
    /* SAFETY: nop does, well, nothing */
    unsafe {
        asm!("nop", options(nostack, nomem));
    }
}
