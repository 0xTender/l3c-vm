use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

#[derive(Debug)]
pub struct Memory {
    data: [u16; 1 << 16],
    pub pc_start: usize,
    pub pc_end: usize,
}

const KEY_BOARD_STATUS: u16 = 0xFE00;
const KEY_BOARD_DATA: u16 = 0xFE02;

impl Memory {
    pub fn load_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        let mut file = File::open(file_path).expect("File not found");

        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut iter = buf.chunks(2);
        let pc_buffer = iter.next().unwrap();

        let mut pc: usize = ((pc_buffer[0] as u16) << 8 | pc_buffer[1] as u16) as usize;

        let pc_start = pc;

        let mut memory = [0u16; 1 << 16];

        for elem in iter {
            let instruction = (elem[0] as u16) << 8 | elem[1] as u16;

            memory[pc] = instruction;

            pc = pc + 1;
        }

        let pc_end = pc;

        Ok(Self {
            data: memory,
            pc_start,
            pc_end,
        })
    }

    pub fn write_memory(&mut self, location: usize, value: u16) {
        self.data[location] = value;
    }

    pub fn read_memory(&mut self, location: u16) -> u16 {
        if location == KEY_BOARD_STATUS {
            let mut buffer = [0; 1];
            std::io::stdin().read_exact(&mut buffer).unwrap();
            println!("Key pressed: {}", buffer[0] as char);

            if buffer[0] != 0 {
                self.data[KEY_BOARD_STATUS as usize] = 1 << 15;
                self.data[KEY_BOARD_DATA as usize] = buffer[0] as u16;
            } else {
                self.data[KEY_BOARD_STATUS as usize] = 0;
            }
        }
        return self.data[location as usize];
    }
}
