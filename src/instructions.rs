#![allow(dead_code)]

use derivative::Derivative;

// one bit
pub type Bit = bool;
// each register is 3 bits
type Register = [Bit; 3];

type Number = u16;
const NUMBER_LENGTH: usize = 16;

pub(crate) fn get_number_from_bits(bit_slice: &[Bit]) -> Number {
    assert!(
        bit_slice.len() <= NUMBER_LENGTH,
        "bit_slice must be NUMBER_LENGTH bits long"
    );
    let mut result: u16 = 0;

    for (i, bit) in bit_slice.iter().enumerate() {
        if *bit {
            result += 1 << i;
        }
    }
    result
}

fn print_bits(val: &[bool], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", get_number_from_bits(val))
}

fn print_bit(val: &bool, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if *val {
        write!(f, "1")
    } else {
        write!(f, "0")
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum LoadType {
    Register { 
        #[derivative(Debug(format_with = "print_bits"))]
        src_register: Register },
    Immediate { 
        #[derivative(Debug(format_with = "print_bits"))]
        value: [Bit; 5] },
}

#[derive(Derivative)]
#[derivative(Debug)]

pub enum JumpType {

    BaseRegister(
        #[derivative(Debug(format_with = "print_bits"))]
        Register
    ),
    Return,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum JumpRegisterType {
    FromOffset {
        #[derivative(Debug(format_with = "print_bits"))]
        p_co_offset_11: [Bit; 11],
    },
    FromRegister {
        #[derivative(Debug(format_with = "print_bits"))]
        base_register: Register,
    },
}

/// OpCode is 16 bits long with last 4 bits storing op-code
#[derive(Derivative)]
#[derivative(Debug)]
pub enum Instructions {
    UnImplemented(u16),
    Branch {
        #[derivative(Debug(format_with = "print_bits"))]
        pc_offset_9: [Bit; 9],
        #[derivative(Debug(format_with = "print_bit"))]
        p: Bit, // 9
        #[derivative(Debug(format_with = "print_bit"))]
        z: Bit, // 10
        #[derivative(Debug(format_with = "print_bit"))]
        n: Bit, // 11
    },
    Add {
        #[derivative(Debug(format_with = "print_bits"))]
        /// store result of addition
        dest_register: Register,
        /// first operand
        #[derivative(Debug(format_with = "print_bits"))]
        src_register: Register,
        /// 5th bit from right is 1 for immediate value
        /// 0 for second source register
        add_type: LoadType,
    },
    // LD
    LoadDirect {
        #[derivative(Debug(format_with = "print_bits"))]
        pc_offset_9: [Bit; 9],
        #[derivative(Debug(format_with = "print_bits"))]
        dest_register: Register,
    },
    // ST
    StoreDirect {
        #[derivative(Debug(format_with = "print_bits"))]
        pc_offset_9: [Bit; 9],
        #[derivative(Debug(format_with = "print_bits"))]
        src_register: Register,
    },
    // JSR | JSRR
    JumpRegister(JumpRegisterType),
    And {
        /// store result of addition
        #[derivative(Debug(format_with = "print_bits"))]
        dest_register: Register,
        /// first operand
        #[derivative(Debug(format_with = "print_bits"))]
        src_register: Register,
        /// 5th bit from right is 1 for immediate value
        /// 0 for second source register
        add_type: LoadType,
    },
    // LDR
    LoadRegister {
        #[derivative(Debug(format_with = "print_bits"))]
        offset6: [Bit; 6],
        #[derivative(Debug(format_with = "print_bits"))]
        base_register: Register,
        #[derivative(Debug(format_with = "print_bits"))]
        dest_register: Register,
    },
    // STR
    StoreRegister {
        #[derivative(Debug(format_with = "print_bits"))]
        offset6: [Bit; 6],
        #[derivative(Debug(format_with = "print_bits"))]
        base_register: Register,
        #[derivative(Debug(format_with = "print_bits"))]
        dest_register: Register,
    },
    // NOT
    Not {
        #[derivative(Debug(format_with = "print_bits"))]
        dest_register: Register,
        #[derivative(Debug(format_with = "print_bits"))]
        src_register: Register,
    },
    // LDI
    LoadIndirect {
        // An address is computed by sign-extending bits [8:0] to
        // 16 bits and adding
        // this value to the incremented PC.
        #[derivative(Debug(format_with = "print_bits"))]
        pc_offset_9: [Bit; 9],
        #[derivative(Debug(format_with = "print_bits"))]
        dest_register: Register,
    },
    // STI
    StoreIndirect {
        // An address is computed by sign-extending bits [8:0] to
        // 16 bits and adding
        // this value to the incremented PC.
        #[derivative(Debug(format_with = "print_bits"))]
        pc_offset_9: [Bit; 9],
        #[derivative(Debug(format_with = "print_bits"))]
        src_register: Register,
    },
    // JMP | RET
    Jump(JumpType),
    // LEA
    LoadEffectiveAddress {
        #[derivative(Debug(format_with = "print_bits"))]
        pc_offset_9: [Bit; 9],
        #[derivative(Debug(format_with = "print_bits"))]
        dest_register: Register,
    },
    // TRAP
    Trap {
        #[derivative(Debug(format_with = "print_bits"))]
        trap_vector: [Bit; 8],
    },
}

impl Instructions {
    // parse instruction
    pub fn parse_instruction(instruction_slice: &[Bit; 16]) -> Instructions {
        // get last 4 bits
        let op_code = get_number_from_bits(&instruction_slice[12..]);
        match op_code {
            0 => {
                let p = instruction_slice[9];
                let z = instruction_slice[10];
                let n = instruction_slice[11];

                Instructions::Branch {
                    pc_offset_9: instruction_slice[0..9].try_into().unwrap(),
                    p,
                    z,
                    n,
                }
            }
            1 => {
                let type_check = instruction_slice[5];

                Instructions::Add {
                    dest_register: instruction_slice[9..12].try_into().unwrap(),
                    src_register: instruction_slice[6..9].try_into().unwrap(),
                    add_type: match type_check {
                        false => LoadType::Register {
                            src_register: instruction_slice[0..3].try_into().unwrap(),
                        },
                        true => LoadType::Immediate {
                            value: instruction_slice[0..5].try_into().unwrap(),
                        },
                    },
                }
            }
            2 => Instructions::LoadDirect {
                pc_offset_9: instruction_slice[0..9].try_into().unwrap(),
                dest_register: instruction_slice[9..12].try_into().unwrap(),
            },
            3 => Instructions::StoreDirect {
                pc_offset_9: instruction_slice[0..9].try_into().unwrap(),
                src_register: instruction_slice[9..12].try_into().unwrap(),
            },
            4 => {
                //
                let type_check = instruction_slice[11];
                match type_check {
                    true => Instructions::JumpRegister(JumpRegisterType::FromOffset {
                        p_co_offset_11: instruction_slice[0..11].try_into().unwrap(),
                    }),
                    false => Instructions::JumpRegister(JumpRegisterType::FromRegister {
                        base_register: instruction_slice[6..9].try_into().unwrap(),
                    }),
                }
            }
            5 => {
                let type_check = instruction_slice[5];

                Instructions::And {
                    dest_register: instruction_slice[9..12].try_into().unwrap(),
                    src_register: instruction_slice[6..9].try_into().unwrap(),
                    add_type: match type_check {
                        false => LoadType::Register {
                            src_register: instruction_slice[0..3].try_into().unwrap(),
                        },
                        true => LoadType::Immediate {
                            value: instruction_slice[0..5].try_into().unwrap(),
                        },
                    },
                }
            }
            6 => Instructions::LoadRegister {
                offset6: instruction_slice[0..6].try_into().unwrap(),
                base_register: instruction_slice[6..9].try_into().unwrap(),
                dest_register: instruction_slice[9..12].try_into().unwrap(),
            },
            7 => Instructions::StoreRegister {
                offset6: instruction_slice[0..6].try_into().unwrap(),
                base_register: instruction_slice[6..9].try_into().unwrap(),
                dest_register: instruction_slice[9..12].try_into().unwrap(),
            },
            9 => Instructions::Not {
                dest_register: instruction_slice[9..12].try_into().unwrap(),
                src_register: instruction_slice[6..9].try_into().unwrap(),
            },
            10 => Instructions::LoadIndirect {
                pc_offset_9: instruction_slice[0..9].try_into().unwrap(),
                dest_register: instruction_slice[9..12].try_into().unwrap(),
            },
            11 => Instructions::StoreIndirect {
                pc_offset_9: instruction_slice[0..9].try_into().unwrap(),
                src_register: instruction_slice[9..12].try_into().unwrap(),
            },
            12 => {
                let base_register: [bool; 3] = instruction_slice[6..9].try_into().unwrap();
                if get_number_from_bits(&base_register) == 7 {
                    Instructions::Jump(JumpType::Return)
                } else {
                    Instructions::Jump(JumpType::BaseRegister(base_register))
                }
            }
            14 => Instructions::LoadEffectiveAddress {
                pc_offset_9: instruction_slice[0..9].try_into().unwrap(),
                dest_register: instruction_slice[9..12].try_into().unwrap(),
            },
            15 => Instructions::Trap {
                trap_vector: instruction_slice[0..8].try_into().unwrap(),
            },
            8 | 13 => Instructions::UnImplemented(op_code),
            _ => panic!("Not implemented {:x}", op_code),
        }
    }
}

// each instruction in 16 bits
pub mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_create() {
        let instruction_slice = &mut [false; 16];
        instruction_slice[15] = true;
        instruction_slice[14] = true;
        instruction_slice[13] = true;
        instruction_slice[12] = false;
        let value = get_number_from_bits(&instruction_slice[12..16]);

        assert_eq!(value, 7);

        instruction_slice[15] = false;
        instruction_slice[14] = true;
        instruction_slice[13] = true;
        instruction_slice[12] = true;
        let value = get_number_from_bits(&instruction_slice[12..16]);

        assert_eq!(value, 14);
    }

    #[test]
    fn test_parse_add() {
        let instruction_slice = &mut [false; 16];
        instruction_slice[15] = true;
        instruction_slice[5] = false;

        let ins = Instructions::parse_instruction(instruction_slice);
        assert!(
            matches!(ins, Instructions::Add { .. }),
            "instruction should be add"
        );
        assert!(
            matches!(
                ins,
                Instructions::Add {
                    add_type: LoadType::Register { .. },
                    ..
                }
            ),
            "instruction should be add register"
        )
    }
}
