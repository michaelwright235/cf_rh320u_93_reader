mod inventory;
mod read;
mod write;
mod stay_quiet;
mod select;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum AccessFlag {
    WithoutUID = 0x02,
    WithUID = 0x22,
    WithoutUIDWithSecurityByte = 0x42
}
