use crate::RAM_SIZE;
use std::sync::Mutex;

pub trait RegArch {}

pub trait CPUArch {
    type Reg: RegArch;
    type Num;

    fn new() -> Self;

    fn set(&mut self, reg: Self::Reg, data: Self::Num);

    fn get(&self, reg: Self::Reg) -> Self::Num;

    fn get_reg8(&self, reg: u8) -> Self::Num;
    fn get_reg16(&self, reg: u8) -> Self::Num;

    fn set_reg8(&mut self, reg: u8, value: Self::Num);
    fn set_reg16(&mut self, reg: u8, value: Self::Num);

    fn dump(&self);

    fn extract_mod_rm(byte: u8) -> (u8, u8, u8);
    fn decode_rm(&mut self, bmod:u8, rm: u8, prefetch: [u8; 4], disp: i16) -> (usize, bool);

    fn read_word(addr: usize, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) -> Self::Num;
    fn read_byte(addr: usize, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>) -> u8;

    fn write_word(content : u16, addr: usize, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>);
    fn write_byte(content: u8, addr: usize, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>);

    fn tick(&mut self, count: u64);

    fn get_ticks(&self) -> u64;

    fn clear_tick(&mut self);

    fn get_ip(&self) -> u16;

    fn ip_inc(&mut self, quant: u16);

    fn decode(&mut self, ram: &'static Mutex<Box<[u8; RAM_SIZE]>>);
}
