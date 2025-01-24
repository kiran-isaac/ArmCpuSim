use crate::binary::*;
use elf::abi::PT_LOAD;
use elf::endian::{AnyEndian, EndianParse};
use elf::ElfBytes;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::Registers;

pub struct Memory {
    pub entrypoint: usize,
    memory: Vec<u8>,
    pub is_little_endian: bool,
}

impl Memory {
    pub fn empty() -> Self {
        Memory {
            entrypoint: 0,
            memory: vec![],
            is_little_endian: true,
        }
    }

    pub fn from_elf(path: &str, regs: &mut Registers) -> Self {
        let path = PathBuf::from(path);
        let file_data = fs::read(path).expect("Could not read file");

        let mut magic_number = Vec::new();
        magic_number.push(file_data[0]);
        magic_number.push(file_data[1]);
        magic_number.push(file_data[2]);
        magic_number.push(file_data[3]);

        let file_data_static = Box::leak(file_data.clone().into_boxed_slice());

        let elf_file = ElfBytes::minimal_parse(file_data_static).expect("Failed to parse ELF file");

        let pt = elf_file.symbol_table().unwrap().unwrap();
        let symtab = pt.0;
        let strtab = pt.1;

        let mut symtab_map = HashMap::new();

        for sym in symtab.iter() {
            symtab_map.insert(strtab.get(sym.st_name as usize).unwrap(), sym.st_value);
        }

        let header: elf::file::FileHeader<AnyEndian> = elf_file.ehdr;
        assert_eq!(header.e_machine, 0x28, "Only ARM architecture is supported");


        let mandatory_sections = vec![
            "__flash",
            "__ram",
            "__flash_size",
            "__ram_size",
            "__stack_size",
        ];
        for section in mandatory_sections {
            if !symtab_map.contains_key(section) {
                panic!("Invalid ELF: Missing symbol: {}", section)
            }
        }

        // Set reg values
        regs.pc = *symtab_map.get("__flash").unwrap() as u32;
        regs.sp =
            (*symtab_map.get("__ram").unwrap() + *symtab_map.get("__ram_size").unwrap()) as u32;

        // Layout memory
        let flash_size = *symtab_map.get("__flash_size").unwrap() as usize;
        let ram_size = *symtab_map.get("__ram_size").unwrap() as usize;
        let mem_size = flash_size + ram_size;
        let mut memory: Vec<u8> = Vec::new();
        memory.resize(mem_size, 0);

        let segment_parse_table = elf_file.segments().unwrap();
        for phdr in segment_parse_table.into_iter() {
            if phdr.p_type != PT_LOAD {
                continue;
            }
            let segment_bytes = elf_file.segment_data(&phdr).unwrap();
            let mut mem_addr = phdr.p_vaddr as usize;

            // #[cfg(debug_assertions)]
            // {
            //     println!("{:X?}", phdr);
            //     println!("{:08X?}, : {:02X?}\n\n", mem_addr, segment_bytes);
            // }
            for byte in segment_bytes {
                memory[mem_addr] = *byte;
                mem_addr += 1;
            }
        }

        Memory {
            entrypoint: elf_file.ehdr.e_entry as usize - 1,
            memory,
            is_little_endian: header.endianness.is_little(),
        }
    }

    /// Dump from vaddr to mem end
    pub fn dump_stack(&self, vsp: u32) -> String {
        let mut s = "STACK DUMP: ".to_string();
        let mut vsp  = vsp;
        while self.mm(vsp) < self.memory.len() as u32 {
            s.push_str(format!("{:02X?}", self.get_byte(vsp)).as_str());
            vsp += 1
        }
        s
    }

    /// Memory Map: Virtual -> Physical
    #[inline(always)]
    pub fn mm(&self, addr: u32) -> u32 {
        if addr >= 0x20000000 {
            addr - 0x20000000 + 0x100000
        } else {
            addr
        }
    }

    pub fn get_instruction(&self, vaddr: u32) -> u32 {
        let hw1 = self.get_halfword(vaddr);

        if matches_mask(hw1, 0b11101 << 11)
            || matches_mask(hw1, 0b11110 << 11)
            || matches_mask(hw1, 0b11111 << 11)
        {
            let hw1 = (hw1 as u32) << 16;
            let hw2 = self.get_halfword(vaddr + 2) as u32;
            hw1 + hw2
        } else {
            hw1 as u32
        }
    }


    pub fn get_byte(&self, vaddr: u32) -> u8 {
        let addr = self.mm(vaddr) as usize;
        if self.is_little_endian {
            self.memory[addr] as u8
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn get_halfword(&self, vaddr: u32) -> u16 {
        let addr = self.mm(vaddr) as usize;
        if self.is_little_endian {
            let lower = self.memory[addr] as u16;
            let upper = self.memory[addr + 1] as u16;
            (upper << 8) + lower
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn get_word(&self, vaddr: u32) -> u32 {
        let addr = self.mm(vaddr) as usize;
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

    pub fn set_word(&mut self, vaddr: u32, value: u32) {
        let addr = self.mm(vaddr) as usize;
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

    pub fn set_halfword(&mut self, vaddr: u32, value: u16) {
        let addr = self.mm(vaddr) as usize;
        if self.is_little_endian {
            self.memory[addr] = (value & 0xff) as u8;
            self.memory[addr + 1] = (value >> 8) as u8;
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn set_byte(&mut self, vaddr: u32, value: u8) {
        let addr = self.mm(vaddr) as usize;
        if self.is_little_endian {
            self.memory[addr] = value;
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }
}
