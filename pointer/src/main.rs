fn main() {
    let a: i64 = 42;

    let a_ptr: *const i64 = &a as *const i64;

    println!("a: {} ({:p})", a, a_ptr);

    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}
