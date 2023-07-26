use std::io;

use crate::instructions::{Bit, get_number_from_bits};


pub const REGISTER_COUNT: usize = 9;

#[derive(Debug)]
pub enum General {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Debug)]
pub enum Registers {
    GeneralRegister(General),
    ProgramCounter, /* program counter */
    Condition,
}

impl Registers {
    pub fn get_register(instruction: &[Bit]) -> io::Result< Registers> {
        let register_count = get_number_from_bits(instruction);
        match register_count {
            0 => Ok(Registers::GeneralRegister(General::R0)),
            1 => Ok(Registers::GeneralRegister(General::R1)),
            2 => Ok(Registers::GeneralRegister(General::R2)),
            3 => Ok(Registers::GeneralRegister(General::R3)),
            4 => Ok(Registers::GeneralRegister(General::R4)),
            5 => Ok(Registers::GeneralRegister(General::R5)),
            6 => Ok(Registers::GeneralRegister(General::R6)),
            7 => Ok(Registers::GeneralRegister(General::R7)),
            8 => Ok(Registers::ProgramCounter),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid register")),
        }
    }
}