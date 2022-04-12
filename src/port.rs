#[cfg(feature="std")]
use std::marker::PhantomData;
#[cfg(not(feature="std"))]
use core::marker::PhantomData;

#[cfg(feature="std")]
use std::arch::asm;

#[cfg(not(feature="std"))]
use core::arch::asm;

/// Trait for limiting [Port] to only being able to read/write u8/16/32.
pub(crate) trait PortRW {
    /// Read a value (self) from the port
    unsafe fn read_port(port: u16) -> Self;
    /// Write a value (self) to the port
    unsafe fn write_port(port: u16, value: Self);
}

// PortRW Implementations //////////////////////////////////////////////////////////////////////////

impl PortRW for u8 {
    unsafe fn read_port(port: u16) -> Self {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }

    unsafe fn write_port(port: u16, value: Self) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortRW for u16 {
    unsafe fn read_port(port: u16) -> Self {
        let value: u16;
        asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }

    unsafe fn write_port(port: u16, value: Self) {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortRW for u32 {
    unsafe fn read_port(port: u16) -> Self {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }

    unsafe fn write_port(port: u16, value: Self) {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

/// A simple wrapper around the asm instructions needed to read/write I/O ports.
pub(crate) struct Port<T: PortRW> {
    addr: u16,
    _phantom: PhantomData<T>,
}
impl<T: PortRW> Port<T> {
    /// Create a new `Port` with the given address and data size
    pub(crate) fn new(addr: u16) -> Self {
        Self { addr, _phantom: PhantomData }
    }

    /// Read a value from the port
    pub(crate) unsafe fn read(&self) -> T {
        T::read_port(self.addr)
    }

    /// Write a value to the port
    pub(crate) unsafe fn write(&self, value: T) {
        T::write_port(self.addr, value);
    }
}
