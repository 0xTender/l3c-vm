use crate::{register::{Registers, REGISTER_COUNT}, memory::Memory, instructions::Instructions};

pub struct VmCPU {
    pub registers: [Registers; REGISTER_COUNT],
    pub memory: Memory,
    pub instructions: Vec<Instructions>,
}

impl VmCPU {}