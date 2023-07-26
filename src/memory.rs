use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

#[derive(Debug)]
pub struct Memory {
    pub data: [u16; 1 << 16],
    pub pc_start: usize,
    pub pc_end: usize,
}

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
}
