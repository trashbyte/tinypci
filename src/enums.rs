/// The major class specification for a PCI device.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// Convert a u8 into the corresponding PciClass
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
    /// Convert a PciClass to its u8 representation
    pub fn as_u8(&self) -> u8 { *self as u8 }
}
impl From<u8> for PciClass {
    /// Convert a u8 into the corresponding PciClass
    fn from(n: u8) -> Self {
        Self::from_u8(n)
    }
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
/// Full class specification (type and subtype) for a PCI device.
///
/// Uses non-camel-case types for readability.
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
    /// Convert a u16 into the corresponding PciFullClass
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
    /// Convert a PciFullClass to its u16 representation
    pub fn as_u16(&self) -> u16 { *self as u16 }
}
impl From<u16> for PciFullClass {
    /// Convert a u16 into the corresponding PciFullClass
    fn from(n: u16) -> Self {
        Self::from_u16(n)
    }
}
