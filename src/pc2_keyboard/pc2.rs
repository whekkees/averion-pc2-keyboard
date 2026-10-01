use crate::arch::x86::io::inb;
use crate::arch::x86::io::outb;


const DATA_PORT : u16 = 0x60;
const STATUS_PORT : u16 = 0x64;

pub fn status() -> u8 { 
    unsafe {
    inb(STATUS_PORT)
    }
}

pub fn read_data () -> u8 { 
    unsafe {
    inb(DATA_PORT)
    }
}

pub unsafe fn write_data (data: u8) { 
    unsafe  {
    wait_input_empty(); 

    outb(DATA_PORT, data);
    }
}

pub fn wait_output_full() {
    while status() & 0x01 == 0 { 

    }
}

pub fn wait_input_empty() {
    while status() & 0x02 != 0 {

    }
}