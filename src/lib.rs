// The MIT License (MIT)
// Copyright (c) 2020 trashbyte
// See LICENSE.txt for full license

use x86_64::instructions::port::Port;

#[cfg(not(feature="std"))] extern crate alloc;
#[cfg(not(feature="std"))] use alloc::vec::Vec;
#[cfg(not(feature="std"))] use alloc::string::String;
#[cfg(not(feature="std"))] use alloc::format;

mod enums;
use enums::*;

pub fn name_for_vendor_id(vendor_id: u16) -> String {
    match vendor_id {
        0x8086 => "Intel Corp. (0x8086)".into(),
        0x1234 => "QEMU (0x1234)".into(),
        _ => format!("Unknown({:#06X})", vendor_id)
    }
}

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
    let mut port = Port::new(0xCF8);
    port.write(address);

    // read data
    let mut port = Port::new(0xCFC);
    port.read()
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
    Port::new(0xCF8).write(address);

    // read data
    Port::new(0xCFC).write(value)
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
