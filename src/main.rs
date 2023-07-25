//! LC-3 architecture
//! Memory
//! 16 bit signed integer => 128KiB (2ˆ16)
//! type u16, size 1 << 16
//!
//! Registers
//! 10 registers of 16 bits
//! 8 general purpose
//! 1 program-counter => address of next instruction in memory
//! 1 condition flag => info about previous register
//!
//! Instruction Set
//! 16 opcodes
//! Each instruction in 16 bits long with last 4 bits storing op-code
//! - OP_BR,     /* branch */
//! - OP_ADD,    /* add  */
//! - OP_LD,     /* load */
//! - OP_ST,     /* store */
//! - OP_JSR,    /* jump register */
//! - OP_AND,    /* bitwise and */
//! - OP_LDR,    /* load register */
//! - OP_STR,    /* store register */
//! - OP_RTI,    /* unused */
//! - OP_NOT,    /* bitwise not */
//! - OP_LDI,    /* load indirect */
//! - OP_STI,    /* store indirect */
//! - OP_JMP,    /* jump */
//! - OP_RES,    /* reserved (unused) */
//! - OP_LEA,    /* load effective address */
//! - OP_TRAP    /* execute trap */
//!
//! Condition Flags
//! FL_POS = 1 << 0, /* + */
//! FL_ZRO = 1 << 1, /* 0 */
//! FL_NEG = 1 << 2, /* - */

use std::{
    fs::File,
    io::{self, Read},
};

use crate::instructions::get_number_from_bits;
use crate::instructions::Instructions;

mod instructions;

fn main() -> io::Result<()> {
    let file_name = "./resources/2048.obj";
    let mut file = File::open(file_name).expect("File not found");

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut iter = buf.chunks(2);
    let pc_buffer = iter.next().unwrap();

    let mut pc: usize = ((pc_buffer[0] as u16) << 8 | pc_buffer[1] as u16) as usize;

    for elem in iter {
        let instruction = (elem[0] as u16) << 8 | elem[1] as u16;

        // convert instruction to array of bits
        let mut instruction_bits = [false; 16];

        let mut i = 0;
        let mut n = instruction;
        while n > 0 {
            instruction_bits[i] = n % 2 == 1;
            n = n / 2;
            i += 1;
        }

        let instruction = Instructions::parse_instruction(&instruction_bits);

        println!("{:?}", instruction);

        pc = pc + 1;
    }

    Ok(())
}
