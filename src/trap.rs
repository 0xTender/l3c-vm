#[derive(Debug)]
pub enum TrapType {
    Get,
    Out,
    Put,
    In,
    PutSp,
    Halt,
}

impl From<u16> for TrapType {
    fn from(value: u16) -> Self {
        match value {
            0x20 => TrapType::Get,
            0x21 => TrapType::Out,
            0x22 => TrapType::Put,
            0x23 => TrapType::In,
            0x24 => TrapType::PutSp,
            0x25 => TrapType::Halt,
            _ => panic!("Invalid trap type {:x}", value),
        }
    }
}
