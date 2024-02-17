use std::{fs::OpenOptions, io::Read};

use memmap2::{Mmap, MmapMut, MmapOptions};

// https://qiita.com/tetsu_koba/items/e1dc088e88979af58ea8
fn map_physical_memory(addr: u64, len: usize) -> MmapMut {
    let map = unsafe {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")
            .unwrap();
        MmapOptions::new()
            .offset(addr as u64)
            .len(len)
            .map_mut(&f)
            .unwrap()
    };
    map
}

fn hex_dump(map: &MmapMut, size: usize) {
    let ptr = map.as_ptr() as *const u32;
    const STEP: usize = 4;
    for y in 0..(size / STEP) {
        let offset_base = y * STEP;
        print!("{:#08x} : ", offset_base);
        println!("{:08x}", unsafe {
            std::ptr::read_volatile(ptr.offset(y.try_into().unwrap()))
        });
    }
}

fn main() {
    // 0x4000.0000(proc addr) <-> 0x1f_0000_0000(Linux VA?)
    let rp1_mem_base: u64 = 0x1f_0000_0000;
    let rp1_pio_offset = 0x0017_8000;
    // let rp1_pio_offset = 0x0000_0000;

    const SIZE: usize = 0x8000;

    let mmap = map_physical_memory(rp1_mem_base + rp1_pio_offset, SIZE);

    println!("map ok");

    println!("read pio registers");
    hex_dump(&mmap, SIZE);
}
