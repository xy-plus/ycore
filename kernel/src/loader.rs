use xmas_elf::{
    program::{Flags, ProgramHeader, SegmentData, Type},
    sections::SectionData,
    ElfFile,
};

fn relocate(elf: &ElfFile, base: usize) -> Result<(), &'static str> {
    let data = elf
        .find_section_by_name(".rela.dyn")
        .ok_or(".rela.dyn not found")?
        .get_data(elf)
        .map_err(|_| "corrupted .rela.dyn")?;
    let entries = match data {
        SectionData::Rela64(entries) => entries,
        _ => return Err("bad .rela.dyn"),
    };
    // let dynsym = elf.dynsym()?;
    println!("{:#x?}", entries);
    for entry in entries {
        println!("{:#x?}", entry);
        // const REL_GOT: u32 = 6;
        // const REL_PLT: u32 = 7;
        // const REL_RELATIVE: u32 = 8;
        // match entry.get_type() {
        //     REL_GOT | REL_PLT => {
        //         let dynsym = &dynsym[entry.get_symbol_table_index() as usize];
        //         let symval = if dynsym.shndx() == 0 {
        //             let name = dynsym.get_name(self)?;
        //             panic!("need to find symbol: {:?}", name);
        //         } else {
        //             base + dynsym.value() as usize
        //         };
        //         let value = symval + entry.get_addend() as usize;
        //         unsafe {
        //             let ptr = (base + entry.get_offset() as usize) as *mut usize;
        //             ptr.write(value);
        //         }
        //     }
        //     REL_RELATIVE => {
        //         let value = base + entry.get_addend() as usize;
        //         unsafe {
        //             let ptr = (base + entry.get_offset() as usize) as *mut usize;
        //             ptr.write(value);
        //         }
        //     }
        //     t => unimplemented!("unknown type: {}", t),
        // }
    }
    Ok(())
}

fn clear_bss(ph: &ProgramHeader) {
    let mem_size = ph.mem_size();
    let file_size = ph.file_size();
    let virt_start_addr = ph.virtual_addr();
    if mem_size > file_size {
        let zero_start = virt_start_addr + file_size;
        unsafe {
            core::ptr::write_bytes(zero_start as *mut u8, 0, (mem_size - file_size) as usize);
        }
    }
}

pub fn load_from_elf(src: &[u8]) -> usize {
    let elf = ElfFile::new(src).unwrap();
    for ph in elf.program_iter() {
        if ph.get_type().unwrap() != Type::Load {
            continue;
        }
        clear_bss(&ph);
        let data = match ph.get_data(&elf).unwrap() {
            SegmentData::Undefined(data) => data,
            _ => {
                error!("???");
                loop {}
            }
        };
        unsafe {
            let dst = core::slice::from_raw_parts_mut(
                ph.virtual_addr() as usize as *mut u8,
                ph.mem_size() as usize,
            );
            dst.copy_from_slice(data);
        }
    }
    return elf.header.pt2.entry_point() as usize;
}
