extern crate capstone;
extern crate keystone;
extern crate unicorn;

use unicorn::{Cpu, CpuX86};
use keystone::OptionType;

const CODE: &'static [u8; 8] = b"\x55\x48\x8b\x05\xb8\x13\x00\x00";

fn main() {
    // capstone
    match capstone::Capstone::new(capstone::Arch::X86, capstone::Mode::LittleEndian) {
        Ok(cs) => {
            //            cs.detail().unwrap();
            cs.att();
            match cs.disasm_all(CODE, 0x1000) {
                Ok(insns) => {
                    println!("Got {} instructions", insns.len());
                    for i in insns.iter() {
                        println!("{}", i);
                        //                        println!("detail: {:?}", i.detail());
                    }
                }
                Err(err) => {
                    println!("Error disassembling: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Error creating disassembler: {}", err);
        }
    }

    // unicorn
    let x86_code32: Vec<u8> = vec![0x41, 0x4a]; // INC ecx; DEC edx

    let emu = CpuX86::new(unicorn::Mode::MODE_32).expect("failed to instantiate emulator");
    let _ = emu.mem_map(0x1000, 0x4000, unicorn::PROT_ALL);
    let _ = emu.mem_write(0x1000, &x86_code32);
    let _ = emu.reg_write_i32(unicorn::RegisterX86::ECX, -10);
    let _ = emu.reg_write_i32(unicorn::RegisterX86::EDX, -50);

    let _ = emu.emu_start(
        0x1000,
        (0x1000 + x86_code32.len()) as u64,
        10 * unicorn::SECOND_SCALE,
        1000,
    );
    assert_eq!(emu.reg_read_i32(unicorn::RegisterX86::ECX), Ok((-9)));
    assert_eq!(emu.reg_read_i32(unicorn::RegisterX86::EDX), Ok((-51)));

    // keystone
    let engine = keystone::Keystone::new(keystone::Arch::X86, keystone::MODE_32)
        .expect("Could not initialize Keystone engine");
    engine
        .option(OptionType::SYNTAX, keystone::OPT_SYNTAX_NASM)
        .expect("Could not set option to nasm syntax");
    let result = engine.asm("mov ah, 0x80".to_string(), 0).expect(
        "Could not assemble",
    );
    let _ = result;
}
