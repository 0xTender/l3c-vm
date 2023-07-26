#![allow(dead_code)]

use crate::instructions::{get_number_from_bits, Bit};
use std::io;

pub const REGISTER_COUNT: usize = 10;

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

impl From<Registers> for usize {
    fn from(value: Registers) -> Self {
        Registers::get_index(value) as usize
    }
}

impl From<u16> for Registers {
    fn from(value: u16) -> Self {
        Registers::get_register(value as u16).unwrap()
    }
}

impl Registers {
    pub fn get_index(r: Registers) -> u16 {
        match r {
            Registers::GeneralRegister(General::R0) => 0,
            Registers::GeneralRegister(General::R1) => 1,
            Registers::GeneralRegister(General::R2) => 2,
            Registers::GeneralRegister(General::R3) => 3,
            Registers::GeneralRegister(General::R4) => 4,
            Registers::GeneralRegister(General::R5) => 5,
            Registers::GeneralRegister(General::R6) => 6,
            Registers::GeneralRegister(General::R7) => 7,
            Registers::ProgramCounter => 8,
            Registers::Condition => 9,
        }
    }

    pub fn get_register(register_count: u16) -> io::Result<Registers> {
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
            9 => Ok(Registers::Condition),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid register")),
        }
    }
}
