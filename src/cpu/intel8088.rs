use std::sync::{Mutex, MutexGuard};

use crate::cpu::arch::{CPUArch, RegArch};

use crate::RAM_SIZE;

#[derive(Debug, Clone, Copy)]
pub enum Reg16 {
    // 8-bit
    Ah,
    Al,
    Bh,
    Bl,
    Ch,
    Cl,
    Dh,
    Dl,
    // 16-bit
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,

    Bp,
    Sp,
    Cs,
    Ds,
    Ss,
    Es,
    Ip,
    Flags,
}

impl RegArch for Reg16 {}

#[derive(Debug)]
pub struct CPU {
    //General purpose
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,

    //Indices & pointers
    si: u16,
    di: u16,

    //Stack
    bp: u16,
    sp: u16,

    //Segment
    cs: u16,
    ds: u16,
    ss: u16,
    es: u16,

    //Instruction P & flags
    ip: u16,
    flags: u16,
}

impl CPUArch for CPU {
    type Num = u16;
    type Reg = Reg16;

    fn new() -> Self {
        CPU {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,

            si: 0,
            di: 0,

            bp: 0,
            sp: 0,

            cs: 0,
            ds: 0,
            ss: 0,
            es: 0,

            ip: 0,
            flags: 0,
        }
    }

    fn set(&mut self, reg: Self::Reg, data: Self::Num) {
        match reg {
            Self::Reg::Ah => self.ax = (self.ax & 0x00FF) | ((data & 0xFF) << 8),
            Self::Reg::Al => self.ax = (self.ax & 0xFF00) | (data & 0xFF),
            Self::Reg::Ax => self.ax = data,

            Self::Reg::Bh => self.bx = (self.bx & 0x00FF) | ((data & 0xFF) << 8),
            Self::Reg::Bl => self.bx = (self.bx & 0xFF00) | (data & 0xFF),
            Self::Reg::Bx => self.bx = data,

            Self::Reg::Ch => self.cx = (self.cx & 0x00FF) | ((data & 0xFF) << 8),
            Self::Reg::Cl => self.cx = (self.cx & 0xFF00) | (data & 0xFF),
            Self::Reg::Cx => self.cx = data,

            Self::Reg::Dh => self.dx = (self.dx & 0x00FF) | ((data & 0xFF) << 8),
            Self::Reg::Dl => self.dx = (self.dx & 0xFF00) | (data & 0xFF),
            Self::Reg::Dx => self.dx = data,

            Self::Reg::Si => self.si = data,
            Self::Reg::Di => self.di = data,
            Self::Reg::Bp => self.bp = data,
            Self::Reg::Sp => self.sp = data,
            Self::Reg::Cs => self.cs = data,
            Self::Reg::Ds => self.ds = data,
            Self::Reg::Ss => self.ss = data,
            Self::Reg::Es => self.es = data,
            Self::Reg::Ip => self.ip = data,
            Self::Reg::Flags => self.flags = data,
        }
    }

    fn get(&self, reg: Self::Reg) -> Self::Num {
        match reg {
            Self::Reg::Ah => (self.ax & 0xFF00) >> 8,
            Self::Reg::Al => self.ax & 0x00FF,
            Self::Reg::Ax => self.ax,

            Self::Reg::Bh => (self.bx & 0xFF00) >> 8,
            Self::Reg::Bl => self.bx & 0x00FF,
            Self::Reg::Bx => self.bx,

            Self::Reg::Ch => (self.cx & 0xFF00) >> 8,
            Self::Reg::Cl => self.cx & 0x00FF,
            Self::Reg::Cx => self.cx,

            Self::Reg::Dh => (self.dx & 0xFF00) >> 8,
            Self::Reg::Dl => self.dx & 0x00FF,
            Self::Reg::Dx => self.dx,

            Self::Reg::Si => self.si,
            Self::Reg::Di => self.di,
            Self::Reg::Bp => self.bp,
            Self::Reg::Sp => self.sp,
            Self::Reg::Cs => self.cs,
            Self::Reg::Ds => self.ds,
            Self::Reg::Ss => self.ss,
            Self::Reg::Es => self.es,
            Self::Reg::Ip => self.ip,
            Self::Reg::Flags => self.flags,
        }
    }

    //IDK how long this will work
    fn get_reg8(&self, reg: u8) -> Self::Num {
        match reg {
            0b000 => self.ax & 0xFF00,
            0b001 => self.cx & 0xFF00,
            0b010 => self.dx & 0xFF00,
            0b011 => self.bx & 0xFF00,
            0b100 => (self.ax & 0x00FF) << 8,
            0b101 => (self.cx & 0x00FF) << 8,
            0b110 => (self.dx & 0x00FF) << 8,
            0b111 => (self.bx & 0x00FF) << 8,
            _ => unreachable!("Invalid register {:b}", reg),
        }
    }

    fn get_reg16(&self, reg: u8) -> Self::Num {
        match reg {
            0b000 => self.ax,
            0b001 => self.cx,
            0b010 => self.dx,
            0b011 => self.bx,
            0b100 => self.sp,
            0b101 => self.bp,
            0b110 => self.si,
            0b111 => self.di,
            _ => unreachable!("Invalid register {:b}", reg),
        }
    }

    //IDK how long this will work
    fn set_reg8(&mut self, reg: u8, value: Self::Num) {
        match reg {
            0b000 => self.ax = (self.ax & 0xFF00) | (value & 0xFF),
            0b001 => self.cx = (self.cx & 0xFF00) | (value & 0xFF),
            0b010 => self.dx = (self.dx & 0xFF00) | (value & 0xFF),
            0b011 => self.bx = (self.bx & 0xFF00) | (value & 0xFF),
            0b100 => self.ax = (self.ax & 0x00FF) | ((value & 0xFF) << 8),
            0b101 => self.cx = (self.cx & 0x00FF) | ((value & 0xFF) << 8),
            0b110 => self.dx = (self.dx & 0x00FF) | ((value & 0xFF) << 8),
            0b111 => self.bx = (self.bx & 0x00FF) | ((value & 0xFF) << 8),
            _ => unreachable!("Invalid register {:b}", reg),
        }
    }

    fn set_reg16(&mut self, reg: u8, value: Self::Num) {
        match reg {
            0b000 => self.ax = value,
            0b001 => self.cx = value,
            0b010 => self.dx = value,
            0b011 => self.bx = value,
            0b100 => self.sp = value,
            0b101 => self.bp = value,
            0b110 => self.si = value,
            0b111 => self.di = value,
            _ => unreachable!("Invalid register {:b}", reg),
        }
    }

    fn dump(&self) {
        println!("DUMP :\n{:#x?}", self);
    }

    fn extract_mod_rm(byte: u8) -> (u8, u8, u8) {
        (
            (byte & 0b11000000) >> 6,
            (byte & 0b00111000) >> 3,
            (byte & 0b00000111),
        )
    }

    //A PIOR implementação que eu já fiz, mas fds, funciona
    fn read_word(pos: u16, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) -> Self::Num {
        let pos = pos as usize;
        let data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();
        let mut buf: [u8; 2] = [0u8; 2];
        buf.copy_from_slice(&data[pos..pos + 2]);

        u16::from_le_bytes(buf)
        //RIP MutexGuard
    }

    fn read_byte(pos: u16, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) -> u8 {
        let data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();
        data[pos as usize]
    }

    fn decode_rm(&mut self, rm: u8, prefetch: [u8; 4]) -> u16 {
        match rm {
            0b000 => self.bx.wrapping_add(self.si),
            0b001 => self.bx.wrapping_add(self.di),
            0b010 => self.bp.wrapping_add(self.si),
            0b011 => self.bp.wrapping_add(self.di),
            0b100 => self.si,
            0b101 => self.di,
            0b110 => {
                self.ip = self.ip.wrapping_add(2);

                let pos: u16 = u16::from_le_bytes([prefetch[2], prefetch[3]]);

                pos
            }
            0b111 => self.bx,
            _ => unreachable!("invalid RM in {:b} {:b}", prefetch[0], prefetch[1]),
        }
    }

    fn decode(&mut self, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) {
        let phisical_ip = (((self.cs as usize) << 4) + (self.ip as usize)) & 0xFFFFF;
        if phisical_ip + 4 > RAM_SIZE {
            panic!("IP out of bounds");
        }

        //Yep, Intel 8088 do the same thing
        //The queue prefetch in 8086 itz 6 bytes, btw
        let prefetch: [u8; 4] = {
            //20 bits wrap-arround
            let data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&data[phisical_ip..phisical_ip + 4]);

            buf
            //RIP MutexGuard
        };

        match prefetch[0] {
            //mov reg16, reg16 - 100010XX
            inst if (inst & 0b111111_00) == 0b100010_00 => {
                println!("instrução mov reg16, reg16");

                // 1 : reg <- R/M
                // 0 : R/M <- reg
                let bit_d = (inst & 0b00000010) >> 1;
                // 0 : Byte
                // 1 : Word
                let bit_w = inst & 0b00000001;

                let (bmod, reg, rm) = CPU::extract_mod_rm(prefetch[1]);

                self.ip = self.ip.wrapping_add(2);

                match bmod {
                    0b00 => {
                        if bit_w == 1 {
                            if bit_d == 1 {
                                let pos: u16 = self.decode_rm(rm, prefetch);

                                let value: u16 = Self::read_word(pos, ram);

                                self.set_reg16(reg, value);
                            } else {
                                let value = self.get_reg16(reg);

                                let value: [u8; 2] = value.to_le_bytes();

                                let addr: u16 = self.decode_rm(rm, prefetch);

                                let mut data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> =
                                    ram.lock().unwrap();

                                //Tinha forma melhor de passar esses dados, mas a minha
                                //preguiça de pesquisar como passar com offset relativo
                                //é maior
                                data[addr as usize] = value[0];
                                data[(addr as usize) + 1] = value[1];

                                //RIP MutexGuard
                            }
                        } else {
                            if bit_d == 1 {
                                let pos: u16 = self.decode_rm(rm, prefetch);

                                let value: u8 = Self::read_byte(pos, ram);

                                self.set_reg8(reg, value as u16);
                            } else {
                                let value: u16 = self.get_reg8(reg);

                                let addr: u16 = self.decode_rm(rm, prefetch);

                                let mut data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> =
                                    ram.lock().unwrap();

                                data[addr as usize] = value as u8;
                                //RIP MutexGuard
                            }
                        }
                    }
                    0b01 => {
                        if bit_w == 1 {
                            if bit_d == 1 { todo!() } else { todo!() }
                        } else {
                            if bit_d == 1 { todo!() } else { todo!() }
                        }
                    }
                    0b10 => {
                        if bit_w == 1 {
                            if bit_d == 1 { todo!() } else { todo!() }
                        } else {
                            if bit_d == 1 { todo!() } else { todo!() }
                        }
                    }
                    0b11 => {
                        if bit_w == 1 {
                            if bit_d == 1 {
                                let value = self.get_reg16(rm);
                                self.set_reg16(reg, value);
                            } else {
                                let value = self.get_reg16(reg);
                                self.set_reg16(rm, value);
                            }
                        } else {
                            if bit_d == 1 {
                                let value = self.get_reg8(rm);
                                self.set_reg8(reg, value);
                            } else {
                                let value = self.get_reg8(reg);
                                self.set_reg8(rm, value);
                            }
                        }
                    }
                    _ => unreachable!("Idk what happend : {:b} {:b}", prefetch[0], prefetch[1]),
                }
            }
            //mov reg, imm - 1011_W_REG
            inst if (inst & 0b1111_0_000) == 0b1011_0_000 => {
                println!("instrução mov reg, imm");

                // W == 1 char
                // W == 0 byte
                let w: u8 = (inst & 0b0000_1_000) >> 3;
                let reg: u8 = inst & 0b00000_111;

                if w == 1 {
                    let value: u16 = u16::from_le_bytes([prefetch[1], prefetch[2]]);
                    self.set_reg16(reg, value);

                    self.ip = self.ip.wrapping_add(3);
                } else {
                    let value: u16 = prefetch[1] as u16;
                    self.set_reg8(reg, value);

                    self.ip = self.ip.wrapping_add(2);
                }
            }
            _ => {
                println!("{:b}", prefetch[0]);
                unimplemented!("Calma pae")
            }
        }
    }
}
