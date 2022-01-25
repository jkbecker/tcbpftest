#![no_std]

#[repr(C)]
pub struct PacketLog {
    pub len: u32,       // packet length
    pub src_addr: u32,  // ipv4 source IP address
    pub dest_addr: u32, // ipv4 destination IP address
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for PacketLog {}
