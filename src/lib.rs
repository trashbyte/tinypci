// The MIT License (MIT)
// Copyright (c) 2020 trashbyte
// See LICENSE.txt for full license

#![doc(html_root_url = "https://docs.rs/tinypci")]

#![feature(asm)]

#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")] use std::fmt::{Display, Formatter, Error};

#[cfg(not(feature="std"))] use core::fmt::{Display, Formatter, Error};
#[cfg(not(feature="std"))] extern crate alloc;
#[cfg(not(feature="std"))] use alloc::vec::Vec;
#[cfg(not(feature="std"))] use alloc::string::String;
#[cfg(not(feature="std"))] use alloc::format;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod enums;
pub use enums::*;


// Port I/O functions //////////////////////////////////////////////////////////

// extracted from the `x86_64` crate.
#[inline]
unsafe fn read_from_port(port: u16) -> u32 {
    let value: u32;
    llvm_asm!("inl %dx, %eax" : "={eax}"(value) : "{dx}"(port) :: "volatile");
    value
}

// extracted from the `x86_64` crate.
#[inline]
unsafe fn write_to_port(port: u16, value: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}


// PciDeviceInfo ///////////////////////////////////////////////////////////////

#[allow(dead_code)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// A struct containing info about a PCI device.
pub struct PciDeviceInfo {
    pub device: u8,
    pub bus: u8,
    pub device_id: u16,
    pub vendor_id: u16,
    pub full_class: PciFullClass,
    pub header_type: u8,
    pub bars: [u32; 6],
    pub supported_fns: [bool; 8],
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
}
impl PciDeviceInfo {
    /// Get the class of the PCI device as a PciClass
    pub fn class(&self) -> PciClass {
        PciClass::from_u8(((self.full_class.as_u16() >> 8) & 0xFF) as u8)
    }
    /// Get the full class of the PCI device as a PciFullClass
    pub fn subclass(&self) -> PciClass {
        PciClass::from_u8((self.full_class.as_u16() & 0xFF) as u8)
    }
}
impl Display for PciDeviceInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let vendor_name = name_for_vendor_id(self.vendor_id);
        writeln!(f, "Device {:X} | Bus {:X} | Vendor: {}", self.device, self.bus, vendor_name)?;
        writeln!(f, "    Class: {:?} ({:#06X})", self.full_class, self.full_class.as_u16())?;
        writeln!(f, "    Header type: {:X}", self.header_type)?;
        write!(f,   "    Supported functions: 0")?;
        for (i, b) in self.supported_fns.iter().enumerate().skip(1) {
            if *b {
                write!(f, ", {}", i)?;
            }
        }
        writeln!(f)?;
        write!(f, "    BARs: [ ")?;
        for i in self.bars.iter() {
            if *i == 0 {
                write!(f, "0x0 ")?;
            }
            else {
                write!(f, "{:#010X} ", i)?;
            }
        }
        writeln!(f, "]")?;
        writeln!(f, "    Interrupt line / pin: {} / {}", self.interrupt_line, self.interrupt_pin)?;
        Ok(())
    }
}


// Public functions ////////////////////////////////////////////////////////////

/// Converts a u16 vendor id into a human-readable name.
pub fn name_for_vendor_id(vendor_id: u16) -> String {
    match vendor_id {
        0x8086 => "Intel Corp. (0x8086)".into(),
        0x1234 => "QEMU (0x1234)".into(),
        _ => format!("Unknown({:#06X})", vendor_id)
    }
}

/// Brute force scans for devices 0-31 on busses 0-255.
pub fn brute_force_scan() -> Vec<PciDeviceInfo> {
    let mut infos = Vec::new();
    for bus in 0u8..=255 {
        for device in 0u8..32 {
            if let Some(info) = check_device(bus, device) {
                infos.push(info);
            }
        }
    }
    infos
}


// Internal functions //////////////////////////////////////////////////////////

fn check_device(bus: u8, device: u8) -> Option<PciDeviceInfo> {
    assert!(device < 32);
    let function = 0u8;

    let (device_id, vendor_id) = get_ids(bus, device, function);
    if vendor_id == 0xFFFF {
        // Device doesn't exist
        return None;
    }

    let class = unsafe { pci_config_read(bus, device, 0, 0x8) };
    let class = (class >> 16) & 0x0000FFFF;
    let pci_class = PciFullClass::from_u16(class as u16);
    let header_type = get_header_type(bus, device, function);

    let mut supported_fns = [true, false, false, false, false, false, false, false];
    if (header_type & 0x80) != 0 {
        // It is a multi-function device, so check remaining functions
        for function in 0u8..8 {
            if get_ids(bus, device, function).1 != 0xFFFF {
                if check_function(bus, device, function) {
                    supported_fns[function as usize] = true;
                }
            }
        }
    }

    let mut bars = [0, 0, 0, 0, 0, 0];
    unsafe {
        bars[0] = pci_config_read(bus, device, 0, 0x10);
        bars[1] = pci_config_read(bus, device, 0, 0x14);
        bars[2] = pci_config_read(bus, device, 0, 0x18);
        bars[3] = pci_config_read(bus, device, 0, 0x1C);
        bars[4] = pci_config_read(bus, device, 0, 0x20);
        bars[5] = pci_config_read(bus, device, 0, 0x24);
    }

    let last_row = unsafe { pci_config_read(bus, device, 0, 0x3C) };

    Some(PciDeviceInfo {
        device, bus, device_id, vendor_id,
        full_class: pci_class,
        header_type,
        bars,
        supported_fns,
        interrupt_line: (last_row & 0xFF) as u8,
        interrupt_pin: ((last_row >> 8) & 0xFF) as u8,
    })
}

unsafe fn pci_config_read(bus: u8, device: u8, func: u8, offset: u8) -> u32 {
    let bus = bus as u32;
    let device = device as u32;
    let func = func as u32;
    let offset = offset as u32;
    // construct address param
    let address = ((bus << 16) | (device << 11) | (func << 8) | (offset & 0xfc) | 0x80000000) as u32;

    // write address
    write_to_port(0xCF8, address);

    // read data
    read_from_port(0xCFC)
}

#[allow(dead_code)]
unsafe fn pci_config_write(bus: u8, device: u8, func: u8, offset: u8, value: u32) {
    let bus = bus as u32;
    let device = device as u32;
    let func = func as u32;
    let offset = offset as u32;
    // construct address param
    let address = ((bus << 16) | (device << 11) | (func << 8) | (offset & 0xfc) | 0x80000000) as u32;

    // write address
    write_to_port(0xCF8, address);

    // write data
    write_to_port(0xCFC, value)
}

fn get_header_type(bus: u8, device: u8, function: u8) -> u8 {
    assert!(device < 32);
    assert!(function < 8);
    let res = unsafe { pci_config_read(bus, device, function, 0x0C) };
    ((res >> 16) & 0xFF) as u8
}

fn check_function(bus: u8, device: u8, function: u8) -> bool {
    assert!(device < 32);
    assert!(function < 8);
    get_ids(bus, device, function).1 != 0xFFFF
}

fn get_ids(bus: u8, device: u8, function: u8) -> (u16, u16) {
    assert!(device < 32);
    assert!(function < 8);
    let res = unsafe { pci_config_read(bus, device, function, 0) };
    let dev_id = ((res >> 16) & 0xFFFF) as u16;
    let vnd_id = (res & 0xFFFF) as u16;
    (dev_id, vnd_id)
}
