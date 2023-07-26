#![allow(dead_code)]

// one bit
pub type Bit = bool;
// each register is 3 bits
type Register = u16;

type Number = u16;
const NUMBER_LENGTH: usize = 16;

fn sign_extend(mut x: Number, bit_count: usize) -> Number {
    if ((x >> (bit_count - 1)) & 1) == 1 {
        x |= 0xFFFF << bit_count;
    }
    return x;
}

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

#[derive(Debug, Clone)]
pub enum LoadType {
    Register { src_register: Register },
    Immediate { value: u16 },
}

#[derive(Debug, Clone)]

pub enum JumpType {
    BaseRegister(Register),
    Return,
}

#[derive(Debug, Clone)]
pub enum JumpRegisterType {
    FromOffset { pc_offset_11: u16 },
    FromRegister { base_register: Register },
}

/// OpCode is 16 bits
/// long with last 4 bits storing op-code
#[derive(Debug, Clone)]
pub enum Instructions {
    UnImplemented(u16),
    Branch {
        pc_offset_9: u16,
        p: Bit, // 9
        z: Bit, // 10
        n: Bit, // 11
    },
    Add {
        /// store result of addition
        dest_register: Register,
        /// first operand
        src_register: Register,
        /// 5th bit from right is 1 for immediate value
        /// 0 for second source register
        add_type: LoadType,
    },
    // LD
    LoadDirect {
        pc_offset_9: u16,

        dest_register: Register,
    },
    // ST
    StoreDirect {
        pc_offset_9: u16,

        src_register: Register,
    },
    // JSR | JSRR
    JumpRegister(JumpRegisterType),
    And {
        /// store result of addition
        dest_register: Register,
        /// first operand
        src_register: Register,
        /// 5th bit from right is 1 for immediate value
        /// 0 for second source register
        add_type: LoadType,
    },
    // LDR
    LoadRegister {
        offset6: u16,

        base_register: Register,

        dest_register: Register,
    },
    // STR
    StoreRegister {
        offset6: u16,

        base_register: Register,

        src_register: Register,
    },
    // NOT
    Not {
        dest_register: Register,

        src_register: Register,
    },
    // LDI
    LoadIndirect {
        // An address is computed by sign-extending bits [8:0] to
        // 16 bits and adding
        // this value to the incremented PC.
        pc_offset_9: u16,

        dest_register: Register,
    },
    // STI
    StoreIndirect {
        // An address is computed by sign-extending bits [8:0] to
        // 16 bits and adding
        // this value to the incremented PC.
        pc_offset_9: u16,

        src_register: Register,
    },
    // JMP | RET
    Jump(JumpType),
    // LEA
    LoadEffectiveAddress {
        pc_offset_9: u16,

        dest_register: Register,
    },
    // TRAP
    Trap {
        trap_vector: u16,
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
                    pc_offset_9: get_number_from_bits(&instruction_slice[0..9]),
                    p,
                    z,
                    n,
                }
            }
            1 => {
                let type_check = instruction_slice[5];

                Instructions::Add {
                    dest_register: get_number_from_bits(&instruction_slice[9..12]),
                    src_register: get_number_from_bits(&instruction_slice[6..9]),
                    add_type: match type_check {
                        false => LoadType::Register {
                            src_register: get_number_from_bits(&instruction_slice[0..3]),
                        },
                        true => LoadType::Immediate {
                            value: sign_extend(get_number_from_bits(&instruction_slice[0..5]), 5),
                        },
                    },
                }
            }
            2 => Instructions::LoadDirect {
                pc_offset_9: sign_extend(get_number_from_bits(&instruction_slice[0..9]), 9),
                dest_register: get_number_from_bits(&instruction_slice[9..12]),
            },
            3 => Instructions::StoreDirect {
                pc_offset_9: get_number_from_bits(&instruction_slice[0..9]),
                src_register: get_number_from_bits(&instruction_slice[9..12]),
            },
            4 => {
                //
                let type_check = instruction_slice[11];
                match type_check {
                    true => Instructions::JumpRegister(JumpRegisterType::FromOffset {
                        pc_offset_11: sign_extend(
                            get_number_from_bits(&instruction_slice[0..11]),
                            11,
                        ),
                    }),
                    false => Instructions::JumpRegister(JumpRegisterType::FromRegister {
                        base_register: get_number_from_bits(&instruction_slice[6..9]),
                    }),
                }
            }
            5 => {
                let type_check = instruction_slice[5];

                Instructions::And {
                    dest_register: get_number_from_bits(&instruction_slice[9..12]),
                    src_register: get_number_from_bits(&instruction_slice[6..9]),
                    add_type: match type_check {
                        false => LoadType::Register {
                            src_register: get_number_from_bits(&instruction_slice[0..3]),
                        },
                        true => LoadType::Immediate {
                            value: get_number_from_bits(&instruction_slice[0..5]),
                        },
                    },
                }
            }
            6 => Instructions::LoadRegister {
                offset6: sign_extend(get_number_from_bits(&instruction_slice[0..6]), 6),
                base_register: get_number_from_bits(&instruction_slice[6..9]),
                dest_register: get_number_from_bits(&instruction_slice[9..12]),
            },
            7 => Instructions::StoreRegister {
                offset6: sign_extend(get_number_from_bits(&instruction_slice[0..6]), 6),
                base_register: get_number_from_bits(&instruction_slice[6..9]),
                src_register: get_number_from_bits(&instruction_slice[9..12]),
            },
            9 => Instructions::Not {
                dest_register: get_number_from_bits(&instruction_slice[9..12]),
                src_register: get_number_from_bits(&instruction_slice[6..9]),
            },
            10 => Instructions::LoadIndirect {
                pc_offset_9: sign_extend( get_number_from_bits(&instruction_slice[0..9]),9),
                dest_register: get_number_from_bits(&instruction_slice[9..12]),
            },
            11 => Instructions::StoreIndirect {
                pc_offset_9: get_number_from_bits(&instruction_slice[0..9]),
                src_register: get_number_from_bits(&instruction_slice[9..12]),
            },
            12 => {
                let base_register: [bool; 3] = instruction_slice[6..9].try_into().unwrap();
                let base_register = get_number_from_bits(&base_register);
                if base_register == 7 {
                    Instructions::Jump(JumpType::Return)
                } else {
                    Instructions::Jump(JumpType::BaseRegister(base_register))
                }
            }
            14 => Instructions::LoadEffectiveAddress {
                pc_offset_9: sign_extend(get_number_from_bits(&instruction_slice[0..9]), 9),
                dest_register: get_number_from_bits(&instruction_slice[9..12]),
            },
            15 => Instructions::Trap {
                trap_vector: get_number_from_bits(&instruction_slice[0..8]),
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
