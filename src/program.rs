use elf::endian::AnyEndian;
use elf::section::SectionHeader;
use elf::ElfBytes;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;


pub struct Program<'data>{
    sh_map : HashMap<String, SectionHeader>,
    bytes : ElfBytes<'data, AnyEndian>,
    _file_data: Vec<u8>,
}

impl<'a> Program<'a> {
    pub fn load(path: &str) -> Self {
        let path = PathBuf::from(path);
        let file_data = fs::read(path).expect("Could not read file.");

        let file_data_static: &'a [u8] = Box::leak(file_data.clone().into_boxed_slice());
        

        let elf_file = ElfBytes::<'a, >::minimal_parse(file_data_static).expect("Failed to parse ELF file");

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

        Program {
            sh_map: with_names,
            bytes: elf_file,
            _file_data: file_data,
        }
    }

    pub fn get_text(&self) -> &'a [u8] {
        let text_sh = self.sh_map.get(".text").expect("Failed to get .text section");

        self.bytes.section_data(text_sh).expect("failed to get elf text section").0
    }
}