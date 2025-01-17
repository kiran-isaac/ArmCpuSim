use elf::endian::{AnyEndian, EndianParse};
use elf::section::SectionHeader;
use elf::ElfBytes;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use elf::file::Class::ELF32;
use crate::instructions::*;
use super::Registers;

pub struct Memory<'data>{
    sh_map : HashMap<String, SectionHeader>,
    bytes : ElfBytes<'data, AnyEndian>,
    _memory: Vec<u8>,
    is_little_endian: bool,
}

impl<'a> Memory<'a> {
    pub fn load_elf(path: &str) -> Self {
        let path = PathBuf::from(path);
        let mut file_data = fs::read(path).expect("Could not read file.");

        let mut magic_number = Vec::new();
        magic_number.push(file_data[0]);
        magic_number.push(file_data[1]);
        magic_number.push(file_data[2]);
        magic_number.push(file_data[3]);

        let file_data_static: &'a [u8] = Box::leak(file_data.clone().into_boxed_slice());
        
        let elf_file = ElfBytes::<'a, >::minimal_parse(file_data_static).expect("Failed to parse ELF file");

        let header = elf_file.ehdr;
        println!("ELF Header: {:?}", header);
        assert_eq!(header.e_machine, 0x28, "Only ARM architecture is supported");

        let (shdrs_opt, strtab_opt) = elf_file
            .section_headers_with_strtab()
            .expect("shdrs offsets should be valid");
        let (shdrs, strtab) = (
            shdrs_opt.expect("Should have shdrs"),
            strtab_opt.expect("Should have strtab")
        );
        // Parse the shdrs and collect them into a map keyed on their zero-copied name
        let with_names: HashMap<String, SectionHeader> = shdrs
            .iter()
            .map(|shdr| {
                (
                    strtab.get(shdr.sh_name as usize).expect("Failed to get section name").to_string(),
                    shdr.clone(),
                )
            })
            .collect();

        let file_data_length = file_data.len();
        file_data.resize(file_data_length + 0x4000, 0);

        Memory {
            sh_map: with_names,
            bytes: elf_file,
            _memory: file_data,
            is_little_endian: header.endianness.is_little(),
        }
    }

    pub fn get_elf_text(&self) -> &'a [u8] {
        let text_sh = self.sh_map.get(".text").expect("Failed to get .text section");

        self.bytes.section_data(text_sh).expect("failed to get elf text section").0
    }

    pub fn set_pc_to_program_start(&self, regs : &mut Registers) {
        regs.pc = self.get_entry_point();
    }

    pub fn get_entry_point(&self) -> u32 {
        self.bytes.ehdr.e_entry as u32 - 1
    }

    pub fn get_instruction(&self, addr: u32) -> u32 {
        let hw1 = self.get_halfword(addr);
        
        if matches_mask(hw1, 0b11101 << 10) || matches_mask(hw1, 0b11110 << 10) || matches_mask(hw1, 0b11111 << 10) {
            let hw1 = (hw1 as u32) << 16;
            let hw2 = self.get_halfword(addr + 2) as u32;
            hw1 + hw2
        } else {
            hw1 as u32
        }
    }

    pub fn get_text_start(&self) -> u32 {
        let text_sh = self.sh_map.get(".text").expect("Failed to get .text section");
        println!("text_sh: {:X?}", text_sh);
        text_sh.sh_addr as u32
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