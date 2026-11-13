use std::sync::{Mutex, MutexGuard};

use crate::cpu::arch::{CPUArch, RegArch};

use crate::RAM_SIZE;

use crate::cpu::mov::mov_inst::*;

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

    //Emulator internals:
    cycles: u64,
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

            cycles: 0,
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
            0b000 => self.ax & 0x00FF,
            0b001 => self.cx & 0x00FF,
            0b010 => self.dx & 0x00FF,
            0b011 => self.bx & 0x00FF,
            0b100 => (self.ax & 0xFF00) >> 8,
            0b101 => (self.cx & 0xFF00) >> 8,
            0b110 => (self.dx & 0xFF00) >> 8,
            0b111 => (self.bx & 0xFF00) >> 8,
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
        //println!("DUMP :\n{:#x?}", self);

        println!(
            "CPU:\nAX: {:#x?}\nBX: {:#x?}\nCX: {:#x?}\nDX: {:#x?}\nSI: {:#x?}\nDI: {:#x?}\nBP: {:#x?}\nSP: {:#x?}\nCS: {:#x?}\nDS: {:#x?}\nSS: {:#x?}\nES: {:#x?}\nIP: {:#x?}\nFLAGS: {:#x?}\n",
            self.ax,
            self.bx,
            self.cx,
            self.dx,
            self.si,
            self.di,
            self.bp,
            self.sp,
            self.cs,
            self.ds,
            self.ss,
            self.es,
            self.ip,
            self.flags
        );
    }

    fn extract_mod_rm(byte: u8) -> (u8, u8, u8) {
        (
            (byte & 0b11000000) >> 6,
            (byte & 0b00111000) >> 3,
            (byte & 0b00000111),
        )
    }

    fn read_word(addr: usize, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) -> Self::Num {
        let data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();
        let mut buf: [u8; 2] = [0u8; 2];
        buf.copy_from_slice(&data[addr..addr + 2]);

        u16::from_le_bytes(buf)
        //RIP MutexGuard
    }

    fn read_byte(addr: usize, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) -> u8 {
        let data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();
        data[addr]
    }

    fn write_word(content: u16, addr: usize, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) {
        let value: [u8; 2] = content.to_le_bytes();

        let mut data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();

        data[addr] = value[0];
        data[addr + 1] = value[1];

        //RIP MutexGuard
    }
    fn write_byte(content: u8, addr: usize, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) {
        let mut data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();

        data[addr] = content;
        //RIP MutexGuard
    }

    fn decode_rm(&mut self, bmod: u8, rm: u8, prefetch: [u8; 4], disp: i16) -> (usize, bool) {
        let mut prefetch_consumed = false;

        let (segment, offset): (u16, u16) = match rm {
            0b110 if bmod == 0b00 => {
                //Nessa operação, o processador SABE que precisa puxar uma WORD logo após o modR/M
                self.ip = self.ip.wrapping_add(2);

                let pos: u16 = u16::from_le_bytes([prefetch[2], prefetch[3]]);

                prefetch_consumed = true;

                (self.ds, pos)
            }

            0b000 => (self.ds, self.bx.wrapping_add(self.si)),
            0b001 => (self.ds, self.bx.wrapping_add(self.di)),
            0b010 => (self.ss, self.bp.wrapping_add(self.si)),
            0b011 => (self.ss, self.bp.wrapping_add(self.di)),
            0b100 => (self.ds, self.si),
            0b101 => (self.ds, self.di),
            0b110 => (self.ss, self.bp),
            0b111 => (self.ds, self.bx),
            _ => unreachable!("invalid RM in {:b} {:b}", prefetch[0], prefetch[1]),
        };

        let ea_cycles: u64 = match (bmod, rm) {
            (0b11, _) => 0,                             // Apenas registrador (sem EA)
            (0b00, 0b110) => 6,                         // Displacement only
            (0b00, 0b000..=0b101) | (0b00, 0b111) => 5, // Base or Index

            // Para mod = 01 (Displacement + Base ou Index)
            (0b01, 0b000..=0b111) => 9,

            // Para mod = 10 (Disp16 + Base + Index, dependendo das combinações)
            (0b10, 0b000) => 11, // BX + SI + disp
            (0b10, 0b001) => 12, // BX + DI + disp
            (0b10, 0b010) => 12, // BP + SI + disp
            (0b10, 0b011) => 11, // BP + DI + disp
            (0b10, 0b100) => 11, // SI + disp
            (0b10, 0b101) => 11, // DI + disp
            (0b10, 0b110) => 12, // BP + disp
            (0b10, 0b111) => 11, // BX + disp

            _ => 5, // Fallback
        };
        self.tick(ea_cycles);

        let final_offset = offset.wrapping_add(disp as u16);

        let physical_addr: u32 = ((segment as u32) << 4) + (final_offset as u32);

        ((physical_addr as usize) % RAM_SIZE, prefetch_consumed)
    }

    fn tick(&mut self, count: u64) {
        self.cycles = self.cycles.wrapping_add(count);
    }

    fn get_ticks(&self) -> u64 {
        self.cycles
    }

    fn clear_tick(&mut self) {
        self.cycles = 0;
    }

     fn get_ip(&self) -> u16 {
        self.ip
     }

    fn ip_inc(&mut self, quant: u16) {
        self.ip = self.ip.wrapping_add(quant);
    }

    fn decode(&mut self, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) {
        let phisical_ip = (((self.cs as usize) << 4) + (self.ip as usize)) & 0xFFFFF;
        if phisical_ip + 4 > RAM_SIZE {
            panic!("IP out of bounds");
        }

        //Yep, Intel 8088 do the same thing
        //The queue prefetch in 8086 itz 6 bytes, btw
        let prefetch: [u8; 4] = {
            let data: MutexGuard<'_, Box<[u8; RAM_SIZE]>> = ram.lock().unwrap();
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&data[phisical_ip..phisical_ip + 4]);

            buf
            //RIP MutexGuard
        };

        match prefetch[0] {
            //mov 100010_DW
            inst if (inst & 0b111111_00) == 0b100010_00 => {
                let tick: u64 = mov_100010_dw(self, prefetch, &ram);

                self.tick(tick);
            }
            //mov 1011_W_REG
            inst if (inst & 0b1111_0_000) == 0b1011_0_000 => {
                let tick: u64 = mov_1011_w_reg(self, prefetch);

                self.tick(tick);
            }

            //mov 1100011_W
            inst if (inst & 0b1111111_0) == 0b1100011_0 => {
                let tick: u64 = mov_1100011_w(self, prefetch, ram);

                self.tick(tick);
            }

            //mov 101000_DW
            inst if (inst & 0b11111100) == 0b101000_00 => {
                let tick: u64 = mov_101000_dw(self, prefetch, ram);

                self.tick(tick);
            }

            _ => {
                todo!(
                    "Unknown instruction : {:#b}. Position: {}",
                    prefetch[0],
                    phisical_ip
                );
            }
        }
    }
}
