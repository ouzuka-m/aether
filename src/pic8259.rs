use x86_64::instructions::port::Port;

pub fn disable() {
    let mut master: Port<u8> = Port::new(0x21);
    let mut slave: Port<u8> = Port::new(0xA1);

    unsafe {
        master.write(0xFF);
        slave.write(0xFF);
    }

    crate::info!("Legacy 8259 PIC disabled");
}
