use crate::binary::*;
use elf::abi::PT_LOAD;
use elf::endian::{AnyEndian, EndianParse};
use elf::ElfBytes;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use super::Registers;
#[derive(Clone)]
pub struct Memory {
    pub entrypoint: usize,
    memory: Vec<u8>,
    pub is_little_endian: bool,
    flash_start: u32,
    flash_size: u32,
    ram_start: u32,
    functions: HashMap<u64, String>,
}

#[derive(Debug)]
pub enum MemError {
    SetOOB,
    LoadOOB,
    SetRO,
}

#[allow(unused)]
impl Memory {
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

        // Get Functions
        let functions: HashMap<u64, String> = symtab
            .iter()
            .filter(|sym| sym.st_symtype() == 2 && sym.st_shndx != 0)
            .map(|sym| {
                (
                    sym.st_value,
                    strtab.get(sym.st_name as usize).unwrap().to_string(),
                )
            })
            .collect();

        // Set reg values
        regs.pc = *symtab_map.get("__flash").unwrap() as u32;
        regs.sp =
            (*symtab_map.get("__ram").unwrap() + *symtab_map.get("__ram_size").unwrap()) as u32;

        // Layout memory
        let flash_start = *symtab_map.get("__flash").unwrap() as u32;
        let flash_size = *symtab_map.get("__flash_size").unwrap() as u32;
        let ram_start = *symtab_map.get("__ram").unwrap() as u32;
        let ram_size = *symtab_map.get("__ram_size").unwrap() as u32;
        let mem_size = (flash_size + ram_size) as usize;

        let mut memory: Vec<u8> = Vec::new();

        // println!("{:?}", symtab_map);

        memory.resize(mem_size, 0);

        let segment_parse_table = elf_file.segments().unwrap();
        for phdr in segment_parse_table.into_iter() {
            if phdr.p_type != PT_LOAD {
                continue;
            }
            let segment_bytes = elf_file.segment_data(&phdr).unwrap();
            let mut mem_addr = phdr.p_paddr as usize;

            for byte in segment_bytes {
                memory[mem_addr] = *byte;
                mem_addr += 1;
            }
        }

        Memory {
            entrypoint: elf_file.ehdr.e_entry as usize - 1,
            memory,
            is_little_endian: header.endianness.is_little(),
            flash_start,
            flash_size,
            ram_start,
            functions,
        }
    }

    pub fn get_function_at(&self, addr: u32) -> Option<&String> {
        self.functions.get(&(addr as u64))
    }

    /// Dump from vaddr to mem end
    pub fn dump_stack(&self, vsp: u32, i_count: u32) -> String {
        let original_vsp = vsp;
        let mut vsp = vsp;

        let mut dump_str = String::new();
        while self.mm(vsp) < self.memory.len() as u32 {
            dump_str.push_str(format!("{:#08X}:{:#08X}: ", vsp - original_vsp, vsp).as_str());
            dump_str.push_str(format!("{:02X?}", self.get_byte_nolog(vsp)).as_str());
            vsp += 1;
            if self.mm(vsp) >= self.memory.len() as u32 {
                break;
            }
            dump_str.push_str(format!("{:02X?}\n", self.get_byte_nolog(vsp)).as_str());
            vsp += 1;
        }
        format!(
            "I: {}, Stack size: {:#X}\n{}\n",
            i_count,
            vsp - original_vsp,
            dump_str
        )
    }

    /// Memory Map: Virtual -> Physical
    #[inline(always)]
    pub fn mm(&self, addr: u32) -> u32 {
        if addr >= self.ram_start {
            addr - self.ram_start + self.flash_size
        } else {
            addr
        }
    }

    pub fn get_byte_nolog(&self, vaddr: u32) -> u8 {
        let addr = self.mm(vaddr) as usize;
        self.memory[addr]
    }

    pub fn get_halfword_nolog(&self, vaddr: u32) -> u16 {
        let addr = self.mm(vaddr) as usize;
        if self.is_little_endian {
            let lower = self.memory[addr] as u16;
            let upper = self.memory[addr + 1] as u16;
            (upper << 8) + lower
        } else {
            unimplemented!("Big endian not supported yet");
        }
    }

    pub fn set_byte_nolog(&mut self, vaddr: u32, value: u8) {
        let addr = self.mm(vaddr) as usize;
        self.memory[addr] = value;
    }

    pub fn get_instruction(&self, vaddr: u32) -> u32 {
        let hw1 = self.get_halfword_nolog(vaddr);

        if matches_mask(hw1, 0b11101 << 11)
            || matches_mask(hw1, 0b11110 << 11)
            || matches_mask(hw1, 0b11111 << 11)
        {
            let hw1 = (hw1 as u32) << 16;
            let hw2 = self.get_halfword_nolog(vaddr + 2) as u32;
            hw1 + hw2
        } else {
            hw1 as u32
        }
    }

    pub fn get_byte(&self, vaddr: u32) -> Result<u8, MemError> {
        let addr = self.mm(vaddr) as usize;
        if addr >= self.memory.len() {
            Err(MemError::LoadOOB)
        } else {
            Ok(self.memory[addr])
        }
    }

    pub fn get_halfword(&self, vaddr: u32) -> Result<u16, MemError> {
        let addr = self.mm(vaddr) as usize;
        if addr >= self.memory.len() - 1 {
            Err(MemError::LoadOOB)
        } else {
            if self.is_little_endian {
                Ok(u16::from_le_bytes([
                    self.memory[addr],
                    self.memory[addr + 1],
                ]))
            } else {
                Ok(u16::from_be_bytes([
                    self.memory[addr],
                    self.memory[addr + 1],
                ]))
            }
        }
    }

    pub fn get_word(&self, vaddr: u32) -> Result<u32, MemError> {
        let addr = self.mm(vaddr) as usize;
        if addr >= self.memory.len() {
            Err(MemError::LoadOOB)
        } else {
            if self.is_little_endian {
                Ok(u32::from_le_bytes([
                    self.memory[addr],
                    self.memory[addr + 1],
                    self.memory[addr + 2],
                    self.memory[addr + 3],
                ]))
            } else {
                Ok(u32::from_be_bytes([
                    self.memory[addr],
                    self.memory[addr + 1],
                    self.memory[addr + 2],
                    self.memory[addr + 3],
                ]))
            }
        }
    }

    pub fn get_word_be(&self, vaddr: u32) -> Result<u32, MemError> {
        let addr = self.mm(vaddr) as usize;
        if addr >= self.memory.len() {
            Err(MemError::LoadOOB)
        } else {
            Ok(u32::from_be_bytes([
                self.memory[addr],
                self.memory[addr + 1],
                self.memory[addr + 2],
                self.memory[addr + 3],
            ]))
        }
    }

    pub fn set_word(&mut self, vaddr: u32, value: u32) -> Result<(), MemError> {
        let addr = self.mm(vaddr) as usize;
        if (addr as u32) < (self.flash_start + self.flash_size) {
            Err(MemError::SetRO)
        } else if addr >= self.memory.len() - 3 {
            Err(MemError::SetOOB)
        } else {
            let bytes = if self.is_little_endian {
                value.to_le_bytes()
            } else {
                value.to_be_bytes()
            };
            self.memory[addr] = bytes[0];
            self.memory[addr + 1] = bytes[1];
            self.memory[addr + 2] = bytes[2];
            self.memory[addr + 3] = bytes[3];
            Ok(())
        }
    }

    pub fn set_halfword(&mut self, vaddr: u32, value: u16) -> Result<(), MemError> {
        let addr = self.mm(vaddr) as usize;
        if (addr as u32) < (self.flash_start + self.flash_size) {
            Err(MemError::SetRO)
        } else if addr >= self.memory.len() - 1 {
            Err(MemError::SetOOB)
        } else {
            let bytes = if self.is_little_endian {
                value.to_le_bytes()
            } else {
                value.to_be_bytes()
            };
            self.memory[addr] = bytes[0];
            self.memory[addr + 1] = bytes[1];
            Ok(())
        }
    }

    pub fn set_byte(&mut self, vaddr: u32, value: u8) -> Result<(), MemError> {
        let addr = self.mm(vaddr) as usize;
        if (addr as u32) < (self.flash_start + self.flash_size) {
            Err(MemError::SetRO)
        } else if addr >= self.memory.len() - 1 {
            Err(MemError::SetOOB)
        } else {
            self.memory[addr] = value;
            Ok(())
        }
    }

    pub fn dump(
        &self,
        width: usize,
        height: usize,
        bottom: usize,
        rows_scrolled_up: usize,
    ) -> String {
        // A byte is two chars, a word is eight
        // Addr is eight byes
        // #aaaaaaaa: 01234567 01234567
        let width_for_mem = width - 11;
        let words_per_line = 4;

        let start_addr = bottom - ((words_per_line * (height)) * 4) - (rows_scrolled_up * 4);
        let mut string = String::new();
        string.reserve(width * height);

        for y in 0..height {
            let line_addr = start_addr + (y * words_per_line * 4);
            string.push_str(&format!("#{:08X?}:", line_addr));
            for x in 0..words_per_line {
                let addr = (line_addr + x * 4) as u32;
                // string.push_str(&format!(" {:08X}", addr));
                if let Ok(word) = self.get_word_be(addr) {
                    string.push_str(&format!(" {:08X}", word));
                } else {
                    string.push_str(" ________");
                }
            }
            string.push('\n');
        }

        string
    }
}
