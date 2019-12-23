pub fn rdtsc() -> u64 {
    unsafe {
        core::arch::x86_64::_mm_lfence();
        core::arch::x86_64::_rdtsc()
    }
}

pub fn sleep(time: u64) {
    let start = rdtsc();
    while rdtsc() - start < time {}
}
