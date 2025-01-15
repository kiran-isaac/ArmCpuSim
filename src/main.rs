use elf::endian::AnyEndian;
use elf::section::SectionHeader;
use elf::ElfBytes;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;


fn main() {
    let path = PathBuf::from("/home/kiran/ACA/thumb/test.elf");
    let file_data = fs::read(path).expect("Could not read file.");
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Failed to parse ELF file");
    let common = file.find_common_data().expect("shdrs should parse");
    // Iterate over section headers to find string tables

    let sh_and_symtab = file.section_headers_with_strtab().unwrap();
    let sh = sh_and_symtab.0.unwrap();
    let symtab = sh_and_symtab.1.unwrap();
    // let first_load_phdr= file.segments().unwrap();

    // dbg!(first_load_phdr);

    // Get the section header table alongside its string table
    let (shdrs_opt, strtab_opt) = file
        .section_headers_with_strtab()
        .expect("shdrs offsets should be valid");
    let (shdrs, strtab) = (
        shdrs_opt.expect("Should have shdrs"),
        strtab_opt.expect("Should have strtab")
    );
    // Parse the shdrs and collect them into a map keyed on their zero-copied name
    let with_names: HashMap<&str, SectionHeader> = shdrs
        .iter()
        .map(|shdr| {
            (
                strtab.get(shdr.sh_name as usize).expect("Failed to get section name"),
                shdr,
            )
        })
        .collect();

    dbg!(with_names);
    // dbg!(symtab);

    let common = file.find_common_data().expect("shdrs should parse");
    // dbg!(common.symtab.unwrap());
    // let (dynsyms, strtab) = (common.dynsyms.unwrap(), common.dynsyms_strs.unwrap());
    // let hash_table = common.sysv_hash.unwrap();

}
