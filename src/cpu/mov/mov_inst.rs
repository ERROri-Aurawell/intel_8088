use std::sync::{Mutex, MutexGuard};

use crate::{CPU, CPUArch, RAM_SIZE};

use crate::cpu::intel8088::Reg16;

/*
pub fn mov_X(
    cpu: &mut CPU,
    prefetch: [u8; 4],
    ram: &'static Mutex<Box<[u8; 1048576]>>,
) -> u64 {}
*/

pub fn mov_1100011_w(
    cpu: &mut CPU,
    prefetch: [u8; 4],
    ram: &'static Mutex<Box<[u8; 1048576]>>,
) -> u64 {
    let bit_w: u8 = prefetch[0] & 0b0000000_1;

    let (bmod, reg, rm) = CPU::extract_mod_rm(prefetch[1]);

    cpu.ip_inc(2);

    if reg != 0 {
        panic!(
            "Invalid Instruction : {:b} {:b}\nReg in ModR/M must be '000'",
            prefetch[0], prefetch[1]
        );
    }

    match bmod {
        0b00 => {
            let (addr, consumed) = cpu.decode_rm(bmod, rm, prefetch, 0);
            //Dependendo do RM, os outros 2 bytes do prefetch são consumidos.
            //quando são, eu preciso fazer um fetch de mais 2 bytes

            if bit_w == 1 {
                let content: u16 = if consumed {
                    let data: MutexGuard<'_, Box<[u8; 1048576]>> = ram.lock().unwrap();

                    let c = u16::from_le_bytes([
                        data[cpu.get_ip() as usize],
                        data[cpu.get_ip() as usize],
                    ]);
                    cpu.ip_inc(2);

                    c
                } else {
                    u16::from_le_bytes([prefetch[2], prefetch[3]])
                };

                CPU::write_word(content, addr, ram);
            } else {
                let content: u8 = if consumed {
                    let data: MutexGuard<'_, Box<[u8; 1048576]>> = ram.lock().unwrap();

                    let c = data[cpu.get_ip() as usize];
                    cpu.ip_inc(1);

                    c
                } else {
                    prefetch[2]
                };

                CPU::write_byte(content, addr, ram);
            }

            10
        }
        0b01 => {
            let disp8: i16 = prefetch[2] as i16;
            cpu.ip_inc(1);
            let (addr, _) = cpu.decode_rm(bmod, rm, prefetch, disp8);

            if bit_w == 1 {
                let byte4: u8 = {
                    let data: MutexGuard<'_, Box<[u8; 1048576]>> = ram.lock().unwrap();

                    let r = data[cpu.get_ip() as usize];
                    cpu.ip_inc(1);

                    r
                    //RIP MutexGuard
                };

                let content = u16::from_le_bytes([prefetch[3], byte4]);

                CPU::write_word(content, addr, ram);
            } else {
                let content: u8 = prefetch[3];

                CPU::write_byte(content, addr, ram);
            }

            10
        }
        0b10 => {
            let disp16: i16 = i16::from_le_bytes([prefetch[2], prefetch[3]]);
            cpu.ip_inc(2);
            let (addr, _) = cpu.decode_rm(bmod, rm, prefetch, disp16);

            let prefetch2: [u8; 2] = {
                let data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();
                let mut buf: [u8; 2] = [0u8; 2];
                buf.copy_from_slice(
                    &data[(cpu.get_ip() as usize)..(cpu.get_ip().wrapping_add(2) as usize)],
                );

                buf
                //RIP MutexGuard
            };
            if bit_w == 1 {
                let content = u16::from_le_bytes(prefetch2);
                cpu.ip_inc(2);

                CPU::write_word(content, addr, ram);
            } else {
                let content = prefetch2[0];
                cpu.ip_inc(1);

                CPU::write_byte(content, addr, ram);
            }

            10
        }
        0b11 => {
            if bit_w == 1 {
                let value: u16 = u16::from_le_bytes([prefetch[2], prefetch[3]]);
                cpu.ip_inc(2);

                cpu.set_reg16(rm, value);
            } else {
                let value: u16 = prefetch[2] as u16;
                cpu.ip_inc(1);

                cpu.set_reg8(rm, value);
            }

            8
        }
        _ => unreachable!("Idk what happend : {:b} {:b}", prefetch[0], prefetch[1]),
    }
}

pub fn mov_1011_w_reg(cpu: &mut CPU, prefetch: [u8; 4]) -> u64 {
    // W == 1 char
    // W == 0 byte
    let w: u8 = (prefetch[0] & 0b0000_1_000) >> 3;
    let reg: u8 = prefetch[0] & 0b00000_111;

    if w == 1 {
        let value: u16 = u16::from_le_bytes([prefetch[1], prefetch[2]]);
        cpu.set_reg16(reg, value);

        cpu.ip_inc(3);
    } else {
        let value: u16 = prefetch[1] as u16;
        cpu.set_reg8(reg, value);

        cpu.ip_inc(2);
    }

    4 //Sempre 4 ciclos
}

pub fn mov_100010_dw(
    cpu: &mut CPU,
    prefetch: [u8; 4],
    ram: &'static Mutex<Box<[u8; 1048576]>>,
) -> u64 {
    // 1 : reg <- R/M
    // 0 : R/M <- reg
    let bit_d = (prefetch[0] & 0b00000010) >> 1;
    // 0 : Byte
    // 1 : Word
    let bit_w = prefetch[0] & 0b00000001;

    let (bmod, reg, rm) = CPU::extract_mod_rm(prefetch[1]);

    //Leu o OPCODE mais o ModR/M
    cpu.ip_inc(2);

    match bmod {
        0b00 => {
            if bit_w == 1 {
                if bit_d == 1 {
                    let (pos, _) = cpu.decode_rm(bmod, rm, prefetch, 0);

                    let value: u16 = CPU::read_word(pos, ram);

                    cpu.set_reg16(reg, value);

                    8
                } else {
                    let value = cpu.get_reg16(reg);

                    let (addr, _) = cpu.decode_rm(bmod, rm, prefetch, 0);

                    CPU::write_word(value, addr, ram);

                    9
                }
            } else {
                if bit_d == 1 {
                    let (addr, _) = cpu.decode_rm(bmod, rm, prefetch, 0);

                    let value: u8 = CPU::read_byte(addr, ram);

                    cpu.set_reg8(reg, value as u16);

                    8
                } else {
                    let value: u16 = cpu.get_reg8(reg);

                    let (addr, _) = cpu.decode_rm(bmod, rm, prefetch, 0);

                    CPU::write_byte(value as u8, addr, ram);

                    9
                }
            }
        }
        0b01 => {
            //Lê o Disp8 logo a frente da instrução
            cpu.ip_inc(1);
            let disp8: i16 = (prefetch[2] as i8) as i16;
            let (addr, _) = cpu.decode_rm(bmod, rm, prefetch, disp8);

            if bit_w == 1 {
                if bit_d == 1 {
                    let value = CPU::read_word(addr, ram);
                    cpu.set_reg16(reg, value);

                    8
                } else {
                    let content = cpu.get_reg16(reg);
                    CPU::write_word(content, addr, ram);

                    9
                }
            } else {
                if bit_d == 1 {
                    let value: u8 = CPU::read_byte(addr, ram);
                    cpu.set_reg8(reg, value as u16);

                    8
                } else {
                    let content: u16 = cpu.get_reg8(reg);
                    CPU::write_byte(content as u8, addr, ram);

                    9
                }
            }
        }
        0b10 => {
            //Lê o Displacement logo a frente da instrução
            cpu.ip_inc(2);
            let disp16: i16 = i16::from_le_bytes([prefetch[2], prefetch[3]]);
            let (addr, _) = cpu.decode_rm(bmod, rm, prefetch, disp16);

            if bit_w == 1 {
                if bit_d == 1 {
                    let value: u16 = CPU::read_word(addr, ram);
                    cpu.set_reg16(reg, value);

                    8
                } else {
                    let content: u16 = cpu.get_reg16(reg);
                    CPU::write_word(content, addr, ram);

                    9
                }
            } else {
                if bit_d == 1 {
                    let value: u8 = CPU::read_byte(addr, ram);
                    cpu.set_reg8(reg, value as u16);

                    8
                } else {
                    let content: u16 = cpu.get_reg8(reg);
                    CPU::write_byte(content as u8, addr, ram);

                    9
                }
            }
        }
        0b11 => {
            if bit_w == 1 {
                if bit_d == 1 {
                    let value = cpu.get_reg16(rm);
                    cpu.set_reg16(reg, value);
                } else {
                    let value = cpu.get_reg16(reg);
                    cpu.set_reg16(rm, value);
                }
            } else {
                if bit_d == 1 {
                    let value = cpu.get_reg8(rm);
                    cpu.set_reg8(reg, value);
                } else {
                    let value = cpu.get_reg8(reg);
                    cpu.set_reg8(rm, value);
                }
            }

            2 //independente de qual seja, são 2 ciclos
        }
        _ => unreachable!("Idk what happend : {:b} {:b}", prefetch[0], prefetch[1]),
    }
}

pub fn mov_101000_dw(
    cpu: &mut CPU,
    prefetch: [u8; 4],
    ram: &'static Mutex<Box<[u8; 1048576]>>,
) -> u64 {
    /*
    10100011 (A3): MOV [addr16], AX
    10100010 (A2): MOV [addr16], AL

    10100001 (A1): MOV AX, [addr16]
    10100000 (A0): MOV AL, [addr16]
    */
    let addr: usize = {
        let relative_addr: u16 = u16::from_le_bytes([prefetch[1], prefetch[2]]);
        //self.ds
        (((cpu.get(Reg16::Ds) as u32) << 4) + (relative_addr as u32)) as usize
    };

    cpu.ip_inc(3);

    let bit_d = (prefetch[0] & 0b00000010) >> 1;
    let bit_w = prefetch[0] & 0b00000001;

    if bit_d == 1 {
        if bit_w == 1 {
            let content: u16 = cpu.get_reg16(0b000);
            CPU::write_word(content, addr, ram);
        } else {
            let content: u8 = cpu.get_reg8(0b000) as u8;
            CPU::write_byte(content, addr, ram);
        }
    } else {
        if bit_w == 1 {
            let value: u16 = CPU::read_word(addr, ram);
            cpu.set_reg16(0b000, value);
        } else {
            let value: u16 = CPU::read_byte(addr, ram) as u16;
            cpu.set_reg8(0b000, value);
        }
    }

    10
}

pub fn mov_10001110(
    cpu: &mut CPU,
    prefetch: [u8; 4],
    ram: &'static Mutex<Box<[u8; 1048576]>>,
) -> u64 {
    //MOV Sreg, r/m16
    let (bmod, reg, rm) = CPU::extract_mod_rm(prefetch[1]);

    //Leu o OPCODE mais o ModR/M
    cpu.ip_inc(2);

    todo!();
}

pub fn mov_10001100(
    cpu: &mut CPU,
    prefetch: [u8; 4],
    ram: &'static Mutex<Box<[u8; 1048576]>>,
) -> u64 {
    //MOV r/m16, Sreg
    let (bmod, reg, rm) = CPU::extract_mod_rm(prefetch[1]);

    //Leu o OPCODE mais o ModR/M
    cpu.ip_inc(2);

    todo!();
}
