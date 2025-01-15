use elf::endian::AnyEndian;
use elf::section::SectionHeader;
use elf::ElfBytes;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;


pub struct Elf{
    sh_map : HashMap<String, SectionHeader>,

}

impl Elf {
    pub fn load(path: &str) -> Self {
        let path = PathBuf::from(path);
        let file_data = fs::read(path).expect("Could not read file.");
        let slice = file_data.as_slice();
        let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Failed to parse ELF file");
        // let common = file.find_common_data().expect("shdrs should parse");

        let (shdrs_opt, strtab_opt) = file
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
                    shdr,
                )
            })
            .collect();

        Elf {
            sh_map : with_names
        }
    }
}