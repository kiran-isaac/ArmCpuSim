use super::Registers;
use crate::instructions::*;
use elf::endian::{AnyEndian, EndianParse};
use elf::section::SectionHeader;
use elf::ElfBytes;
use std::collections::HashMap;
use std::{fs, os};
use std::path::PathBuf;

pub struct Memory {
    pub os_entrypoint: usize,
    _memory: Vec<u8>,
    pub is_little_endian: bool,
}

impl Memory {
    pub fn from_os_elf(path: &str) -> Self {
        let path = PathBuf::from(path);
        let mut file_data = fs::read(path).expect("Could not read file");

        let mut magic_number = Vec::new();
        magic_number.push(file_data[0]);
        magic_number.push(file_data[1]);
        magic_number.push(file_data[2]);
        magic_number.push(file_data[3]);

        let file_data_static= Box::leak(file_data.clone().into_boxed_slice());

        let elf_file =
            ElfBytes::minimal_parse(file_data_static).expect("Failed to parse ELF file");

        let header: elf::file::FileHeader<AnyEndian> = elf_file.ehdr;
        assert_eq!(header.e_machine, 0x28, "Only ARM architecture is supported");

        let file_data_length = file_data.len();
        file_data.resize(file_data_length + 0x4000, 0);

        Memory {
            os_entrypoint: elf_file.ehdr.e_entry as usize - 1,
            _memory: file_data,
            is_little_endian: header.endianness.is_little(),
        }
    }

    pub fn load_additional_elf(&mut self, path: &str) -> usize {
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

        let header: elf::file::FileHeader<AnyEndian> = elf_file.ehdr;
        assert_eq!(header.e_machine, 0x28, "Only ARM architecture is supported");

        let file_data_length = file_data.len();
        let offset = self._memory.len();
        self._memory.resize(offset + file_data_length, 0);

        for i in 0..file_data_length {
            self._memory[offset + i] = file_data[i];
        }

        elf_file.ehdr.e_entry as usize - 1
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

    pub fn get_halfword(&self, addr: u32) -> u16 {
        let addr = addr as usize;
        if self.is_little_endian {
            let lower = self._memory[addr] as u16;
            let upper = self._memory[addr + 1] as u16;
            (upper << 8) + lower
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn get_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        if self.is_little_endian {
            u32::from_le_bytes([
                self._memory[addr + 2],
                self._memory[addr + 3],
                self._memory[addr],
                self._memory[addr + 1],
            ])
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }
}
