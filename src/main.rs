use std::sync::{LazyLock, Mutex, MutexGuard};

mod cpu;
use cpu::arch::CPUArch;
use cpu::intel8088::{CPU, Reg16};

pub const RAM_SIZE: usize = 1024 * 1024;

//Puta gambiarra. Box<[u8; RAM_SIZE] só joga direto no heap no nível de otimização 1
//fora do modo debug. Sem isso o programa crasha por stack overflow
static RAM: LazyLock<Mutex<Box<[u8; RAM_SIZE]>>> = LazyLock::new(|| {
    let vec_ram: Vec<u8> = vec![0u8; RAM_SIZE];
    
    let boxed_slice: Box<[u8]> = vec_ram.into_boxed_slice();
    let boxed_array: Box<[u8; RAM_SIZE]> = boxed_slice.try_into().unwrap();

    Mutex::new(boxed_array)
});

fn main() {
    println!("Hello, Intel 8088!");

    let mut cpu = CPU::new();

    //cpu.set(Reg16::Ah, 0x40);
    //cpu.set(Reg16::Al, 0x10);

    //let get_h = cpu.get(Reg16::Ah);
    //let get_l = cpu.get(Reg16::Al);

    //println!("High-Low : {:#x}-{:#x}", get_h, get_l); // High-Low : 0x40-0x10
    
    cpu.dump();

    {
        /*
            ;0xB8,0x0E,0x00,0x89,0xC3,0x89,0xD9,0x89,0xC8

            cpu 8086
            bits 16

            start:
                mov ax, 0x0E
                mov bx, ax
                mov cx, bx
                mov ax, cx

            ;yep, useless code, just mov reg operations
        */
        const INSTRUCTIONS: [u8; 9] = [
            0b10111000, 0b00001110, 0b00000000, 0b10001001, 0b11000011, 0b10001001, 0b11011001,
            0b10001001, 0b11001000,
        ];

        let mut ram: MutexGuard<'_, Box<[u8; 1048576]>> = RAM.lock().unwrap();

        ram[..INSTRUCTIONS.len()].copy_from_slice(&INSTRUCTIONS);
    }

    cpu.decode(&RAM);
    cpu.dump();

    cpu.decode(&RAM);
    cpu.dump();
}

/*
Talvez eu precise usar isso pra evitar stack overflow

[profile.dev]
opt-level = 1
*/
