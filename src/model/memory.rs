use crate::binary::*;
use elf::endian::{AnyEndian, EndianParse};
use elf::{file, symbol, ElfBytes};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct Memory {
    pub entrypoint: usize,
    memory: Vec<u8>,
    pub is_little_endian: bool,
}

impl Memory {
    pub fn empty() -> Self {
        Memory {entrypoint: 0, memory: vec![], is_little_endian: true}
    }
    
    pub fn from_elf(path: &str) -> Self {
        let path = PathBuf::from(path);
        let file_data = fs::read(path).expect("Could not read file");

        let mut magic_number = Vec::new();
        magic_number.push(file_data[0]);
        magic_number.push(file_data[1]);
        magic_number.push(file_data[2]);
        magic_number.push(file_data[3]);

        let file_data_static= Box::leak(file_data.clone().into_boxed_slice());

        let elf_file =
            ElfBytes::minimal_parse(file_data_static).expect("Failed to parse ELF file");

        let segment_parse_table = elf_file.segments().unwrap();
        for phdr in segment_parse_table.into_iter() {
            println!("{:08X?}", phdr);
        }

        let pt = elf_file.symbol_table().unwrap().unwrap();
        let symtab = pt.0;
        let strtab = pt.1;

        let mut symtab_map = HashMap::new();

        for sym in symtab.iter() {
            symtab_map.insert(strtab.get(sym.st_name as usize).unwrap(), sym.st_value);
        }

        println!("{:08X?}", symtab_map);

        let header: elf::file::FileHeader<AnyEndian> = elf_file.ehdr;
        assert_eq!(header.e_machine, 0x28, "Only ARM architecture is supported");

        let mut memory = file_data;
        // Add 4kb to memory
        memory.resize(memory.len() + 0x4000, 0);

        Memory {
            entrypoint: elf_file.ehdr.e_entry as usize - 1,
            memory,
            is_little_endian: header.endianness.is_little(),
        }
    }

    pub fn get_instruction(&self, addr: u32) -> u32 {
        let hw1 = self.get_halfword(addr);

        if matches_mask(hw1, 0b11101 << 10)
            || matches_mask(hw1, 0b11110 << 10)
            || matches_mask(hw1, 0b11111 << 10)
        {
            let hw1 = (hw1 as u32) << 16;
            let hw2 = self.get_halfword(addr + 2) as u32;
            hw1 + hw2
        } else {
            hw1 as u32
        }
    }
    pub fn get_byte(&self, addr: u32) -> u8 {
        let addr = addr as usize;
        if self.is_little_endian {
            self.memory[addr] as u8
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn get_halfword(&self, addr: u32) -> u16 {
        let addr = addr as usize;
        if self.is_little_endian {
            let lower = self.memory[addr] as u16;
            let upper = self.memory[addr + 1] as u16;
            (upper << 8) + lower
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn get_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        if self.is_little_endian {
            u32::from_le_bytes([
                self.memory[addr + 2],
                self.memory[addr + 3],
                self.memory[addr],
                self.memory[addr + 1],
            ])
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn set_word(&mut self, addr: u32, value: u32) {
        let addr = addr as usize;
        if self.is_little_endian {
            let bytes = value.to_le_bytes();
            self.memory[addr] = bytes[2];
            self.memory[addr + 1] = bytes[3];
            self.memory[addr + 2] = bytes[0];
            self.memory[addr + 3] = bytes[1];
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn set_halfword(&mut self, addr: u32, value: u16) {
        let addr = addr as usize;
        if self.is_little_endian {
            self.memory[addr] = (value & 0xff) as u8;
            self.memory[addr + 1] = (value >> 8) as u8;
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn set_byte(&mut self, addr: u32, value: u8) {
        let addr = addr as usize;
        if self.is_little_endian {
            self.memory[addr] = value;
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }
}
