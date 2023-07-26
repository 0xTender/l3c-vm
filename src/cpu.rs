use crate::{
    instructions::{Instructions, LoadType},
    memory::Memory,
    register::{General, Registers, REGISTER_COUNT},
    trap::TrapType,
};

#[derive(Debug)]
pub struct VmCPU {
    pub registers: [u16; REGISTER_COUNT],
    pub memory: Memory,
}

const FL_POS: u16 = 1 << 0; /* P */
const FL_ZRO: u16 = 1 << 1; /* Z */
const FL_NEG: u16 = 1 << 2; /* N */

impl VmCPU {
    pub fn new(mut registers: [u16; REGISTER_COUNT], memory: Memory) -> Self {
        let pc_register: usize = Registers::ProgramCounter.into();
        registers[pc_register] = memory.pc_start as u16;

        // for elem in memory.data[memory.pc_start..memory.pc_end].iter() {
        //     let mut instruction_bits = [false; 16];

        //     let mut i = 0;
        //     let mut n = *elem;
        //     while n > 0 {
        //         instruction_bits[i] = n % 2 == 1;
        //         n = n / 2;
        //         i += 1;
        //     }
        //     instructions.push(Instructions::parse_instruction(&instruction_bits));
        // }

        Self { registers, memory }
    }

    pub fn read_register(&self, register: Registers) -> u16 {
        let register_index: usize = register.into();
        self.registers[register_index]
    }

    pub fn update_register(&mut self, register: Registers, value: u16) {
        let register_index: usize = register.into();
        self.registers[register_index] = value;
    }

    pub fn get_instruction(&mut self) -> Instructions {
        // let elem = self.memory.data[];
        let memory_location: u16 = self.read_register(Registers::ProgramCounter);

        let elem = self.memory.data[memory_location as usize];

        let mut instruction_slice = [false; 16];

        let mut i = 0;
        let mut n = elem;
        while n > 0 {
            instruction_slice[i] = n % 2 == 1;
            n = n / 2;
            i += 1;
        }

        self.update_register(Registers::ProgramCounter, memory_location + 1);
        let i = Instructions::parse_instruction(&instruction_slice);
        println!("\t\t{:} {:?}", memory_location, i);
        i
    }

    pub fn update_flag(&mut self, register_index: u16) {
        let register = self.registers[register_index as usize];

        if register == 0 {
            self.update_register(Registers::Condition, FL_ZRO);
        } else if register >> 15 == 1 {
            self.update_register(Registers::Condition, FL_NEG);
        } else {
            self.update_register(Registers::Condition, FL_POS);
        }
    }

    #[allow(unused_variables)]
    pub fn execute(&mut self) {
        loop {
            let instruction = self.get_instruction();

            match instruction {
                Instructions::UnImplemented(_) => todo!("UnImplemented"),
                Instructions::Branch {
                    pc_offset_9,
                    p,
                    z,
                    n,
                } => {}
                Instructions::Add {
                    dest_register,
                    src_register,
                    add_type,
                } => match add_type {
                    LoadType::Register { src_register } => todo!(),
                    LoadType::Immediate { value } => {
                        //
                        let operand = self.read_register(src_register.into());

                        self.update_register(dest_register.into(), operand.wrapping_add(value))
                    }
                },
                Instructions::LoadDirect {
                    pc_offset_9,
                    dest_register,
                } => {
                    let pc_value = self.read_register(Registers::ProgramCounter);

                    let value = self.memory.data[(pc_value.wrapping_add(pc_offset_9)) as usize];

                    self.update_register(dest_register.into(), value);

                    self.update_flag(dest_register);
                }
                Instructions::StoreDirect {
                    pc_offset_9,
                    src_register,
                } => todo!("StoreDirect"),
                Instructions::JumpRegister(register_type) => {
                    match register_type {
                        crate::instructions::JumpRegisterType::FromOffset { pc_offset_11 } => {
                            //
                            let pc_value = self.read_register(Registers::ProgramCounter);
                            self.update_register(Registers::GeneralRegister(General::R7), pc_value);

                            self.update_register(
                                Registers::ProgramCounter,
                                pc_value.wrapping_add(pc_offset_11),
                            );
                        }
                        crate::instructions::JumpRegisterType::FromRegister { base_register } => {
                            todo!("register")
                        }
                    }
                }
                Instructions::And {
                    dest_register,
                    src_register,
                    add_type,
                } => {
                    let base: u16 = self.read_register(src_register.into());
                    match add_type {
                        LoadType::Register { src_register } => {
                            let value = self.read_register(src_register.into());
                            self.update_register(dest_register.into(), base & value);
                            self.update_flag(dest_register.into());
                        }
                        LoadType::Immediate { value } => {
                            self.update_register(dest_register.into(), base & value);
                            self.update_flag(dest_register.into())
                        },
                    }
                },
                Instructions::LoadRegister {
                    offset6,
                    base_register,
                    dest_register,
                } => {
                    //
                    let base = self.read_register(base_register.into());
                    let memory_location = base.wrapping_add(offset6);

                    let value = self.memory.data[memory_location as usize];

                    self.update_register(dest_register.into(), value);
                    self.update_flag(dest_register.into());
                }
                Instructions::StoreRegister {
                    offset6,
                    base_register,
                    src_register: dest_register,
                } => {
                    //
                    let base = self.read_register(base_register.into());

                    self.memory.data[(base.wrapping_add(offset6)) as usize] =
                        self.read_register(dest_register.into());
                }
                Instructions::Not {
                    dest_register,
                    src_register,
                } => todo!("Not"),
                Instructions::LoadIndirect {
                    pc_offset_9,
                    dest_register,
                } => {
                    //
let pc_value=                    self.read_register(Registers::ProgramCounter);
                    let indirect_memory_location = pc_value.wrapping_add(pc_offset_9) as usize;
                    let direct_value = self.memory.data[self.memory.data[indirect_memory_location] as usize];

                    self.update_register(dest_register.into(), direct_value);
                    self.update_flag(dest_register);

                },
                Instructions::StoreIndirect {
                    pc_offset_9,
                    src_register,
                } => todo!("StoreIndirect"),
                Instructions::Jump(_) => todo!("Jump"),
                Instructions::LoadEffectiveAddress {
                    pc_offset_9,
                    dest_register,
                } => {
                    //
                    self.update_register(
                        dest_register.into(),
                        self.read_register(Registers::ProgramCounter)
                            .wrapping_add(pc_offset_9),
                    );
                    self.update_flag(dest_register);
                }
                Instructions::Trap { trap_vector } => {
                    let trap: TrapType = trap_vector.into();

                    self.update_register(
                        Registers::GeneralRegister(General::R7),
                        self.read_register(Registers::ProgramCounter),
                    );

                    // read from R_R0
                    match trap {
                        TrapType::Put => {
                            let mut memory_start =
                                self.read_register(Registers::GeneralRegister(General::R0));
                            let mut character = self.memory.data[memory_start as usize];

                            let mut from_u32 = std::char::from_u32(character as u32).unwrap();

                            let mut chars = Vec::new();

                            while from_u32 != '\0' {
                                chars.push(from_u32);

                                memory_start = memory_start + 1;
                                character = self.memory.data[memory_start as usize];

                                from_u32 = std::char::from_u32(character as u32).unwrap();
                            }

                            let string: String = chars.into_iter().collect();
                            println!("{}", string);
                        }
                        _ => todo!("Trap"),
                    }
                }
            }
        }
    }
}
