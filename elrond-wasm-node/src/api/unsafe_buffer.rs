use core::arch::wasm32;

const BUFFER_SIZE: usize = 32;

/// A static mutable buffer acting as temporary storage for certain operations,
/// such as handling temporary big uint representations.
/// Highly unsafe, use with caution.
///
/// It doesn't matter what we initialize with, since it needs to be cleared before each use.
static mut BUFFER: [u8; BUFFER_SIZE] = [b'u'; BUFFER_SIZE];

pub(crate) unsafe fn clear_buffer() {
    core::ptr::write_bytes(BUFFER.as_mut_ptr(), 0u8, BUFFER_SIZE);
}

pub(crate) unsafe fn buffer_ptr() -> *mut u8 {
    BUFFER.as_mut_ptr()
}

const MY_STRING: &[u8] = b"lorem ipostum";

pub(crate) unsafe fn do_thingy() {
    wasm32::memory_grow(0, 64);
    core::ptr::write_bytes(MY_STRING, 0u8, BUFFER_SIZE);
}
