#[cfg(feature="std")] use std::fmt::{Display, Formatter, Error};
#[cfg(not(feature="std"))] use core::fmt::{Display, Formatter, Error};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum PciClass {
    Unclassified = 0x00,
    MassStorage = 0x01,
    Network = 0x02,
    Display = 0x03,
    Multimedia = 0x04,
    Memory = 0x05,
    Bridge = 0x06,
    Other = 0xFF,
}
impl PciClass {
    pub fn from_u8(n: u8) -> PciClass {
        match n {
            0x00 => PciClass::Unclassified,
            0x01 => PciClass::MassStorage,
            0x02 => PciClass::Network,
            0x03 => PciClass::Display,
            0x04 => PciClass::Multimedia,
            0x05 => PciClass::Memory,
            0x06 => PciClass::Bridge,
            _ => PciClass::Other
        }
    }
    pub fn as_u8(&self) -> u8 { *self as u8 }
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum PciFullClass {
    Unclassified_NonVgaCompatible = 0x0000,
    Unclassified_VgaCompatible = 0x0001,

    MassStorage_ScsiBus = 0x0100,
    MassStorage_IDE = 0x0101,
    MassStorage_Floppy = 0x0102,
    MassStorage_IpiBus = 0x0103,
    MassStorage_RAID = 0x0104,
    MassStorage_ATA = 0x0105,
    MassStorage_SATA = 0x0106,
    MassStorage_SerialSCSI = 0x0107,
    MassStorage_NVM = 0x0108,
    MassStorage_Other = 0x0180,

    Network_Ethernet = 0x0200,
    Network_TokenRing = 0x0201,
    Network_FDDI = 0x0202,
    Network_ATM = 0x0203,
    Network_ISDN = 0x0204,
    Network_WorldFlip = 0x0205,
    Network_PICMG = 0x0206,
    Network_Infiniband = 0x0207,
    Network_Fabric = 0x0208,
    Network_Other = 0x0280,

    Display_VGA = 0x0300,
    Display_XGA = 0x0301,
    Display_3D = 0x0302,
    Display_Other = 0x0380,

    Multimedia_Video = 0x0400,
    Multimedia_AudioController = 0x0401,
    Multimedia_Telephony = 0x0402,
    Multimedia_AudioDevice = 0x0403,
    Multimedia_Other = 0x0480,

    Memory_RAM = 0x0500,
    Memory_Flash = 0x0501,
    Memory_Other = 0x0580,

    Bridge_Host = 0x0600,
    Bridge_ISA = 0x0601,
    Bridge_EISA = 0x0602,
    Bridge_MCA = 0x0603,
    Bridge_PciToPci = 0x0604,
    Bridge_PCMCIA = 0x0605,
    Bridge_NuBus = 0x0606,
    Bridge_CardBus = 0x0607,
    Bridge_RACEway = 0x0608,
    Bridge_PciToPciSemiTransparent = 0x0609,
    Bridge_InfinibandToPci = 0x060A,
    Bridge_Other = 0x0680,

    Unknown = 0xFFFF,
}
impl PciFullClass {
    // listen, i know this sucks, but i didn't want to include
    // `num`, `num-traits` and `num-derive` as dependencies for
    // this crate just for a convenience function
    pub fn from_u16(n: u16) -> PciFullClass {
        match n {
            0x0000 => PciFullClass::Unclassified_NonVgaCompatible,
            0x0001 => PciFullClass::Unclassified_VgaCompatible,

            0x0100 => PciFullClass::MassStorage_ScsiBus,
            0x0101 => PciFullClass::MassStorage_IDE,
            0x0102 => PciFullClass::MassStorage_Floppy,
            0x0103 => PciFullClass::MassStorage_IpiBus,
            0x0104 => PciFullClass::MassStorage_RAID,
            0x0105 => PciFullClass::MassStorage_ATA,
            0x0106 => PciFullClass::MassStorage_SATA,
            0x0107 => PciFullClass::MassStorage_SerialSCSI,
            0x0108 => PciFullClass::MassStorage_NVM,
            0x0180 => PciFullClass::MassStorage_Other,

            0x0200 => PciFullClass::Network_Ethernet,
            0x0201 => PciFullClass::Network_TokenRing,
            0x0202 => PciFullClass::Network_FDDI,
            0x0203 => PciFullClass::Network_ATM,
            0x0204 => PciFullClass::Network_ISDN,
            0x0205 => PciFullClass::Network_WorldFlip,
            0x0206 => PciFullClass::Network_PICMG,
            0x0207 => PciFullClass::Network_Infiniband,
            0x0208 => PciFullClass::Network_Fabric,
            0x0280 => PciFullClass::Network_Other,

            0x0300 => PciFullClass::Display_VGA,
            0x0301 => PciFullClass::Display_XGA,
            0x0302 => PciFullClass::Display_3D,
            0x0380 => PciFullClass::Display_Other,

            0x0400 => PciFullClass::Multimedia_Video,
            0x0401 => PciFullClass::Multimedia_AudioController,
            0x0402 => PciFullClass::Multimedia_Telephony,
            0x0403 => PciFullClass::Multimedia_AudioDevice,
            0x0480 => PciFullClass::Multimedia_Other,

            0x0500 => PciFullClass::Memory_RAM,
            0x0501 => PciFullClass::Memory_Flash,
            0x0580 => PciFullClass::Memory_Other,

            0x0600 => PciFullClass::Bridge_Host,
            0x0601 => PciFullClass::Bridge_ISA,
            0x0602 => PciFullClass::Bridge_EISA,
            0x0603 => PciFullClass::Bridge_MCA,
            0x0604 => PciFullClass::Bridge_PciToPci,
            0x0605 => PciFullClass::Bridge_PCMCIA,
            0x0606 => PciFullClass::Bridge_NuBus,
            0x0607 => PciFullClass::Bridge_CardBus,
            0x0608 => PciFullClass::Bridge_RACEway,
            0x0609 => PciFullClass::Bridge_PciToPciSemiTransparent,
            0x060A => PciFullClass::Bridge_InfinibandToPci,
            0x0680 => PciFullClass::Bridge_Other,

            _ => PciFullClass::Unknown
        }
    }
    pub fn as_u16(&self) -> u16 { *self as u16 }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
    pub fn class(&self) -> PciClass {
        PciClass::from_u8(((self.full_class.as_u16() >> 8) & 0xFF) as u8)
    }
    pub fn subclass(&self) -> PciClass {
        PciClass::from_u8((self.full_class.as_u16() & 0xFF) as u8)
    }
}
impl Display for PciDeviceInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let vendor_name = super::name_for_vendor_id(self.vendor_id);
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
