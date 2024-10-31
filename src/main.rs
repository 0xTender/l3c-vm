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

use cpu::VmCPU;

use crate::memory::Memory;

mod cpu;
mod instructions;
mod memory;
mod register;
mod trap;

fn main() -> std::io::Result<()> {
    let file_name = "./resources/rogue.obj";
    let memory = Memory::load_from_file(file_name)?;

    let mut vm = VmCPU::new([0; 10], memory);

    vm.execute();

    Ok(())
}
