extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;
use std::io::Cursor;

type Op = Fn(u16, &u16, &[u16], u16, u16);

const FLAG_CARRY: u16 = 0b0001;
const FLAG_ZERO:  u16 = 0b0010;

fn op_mov(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    memory[addr] = value;

    return ip;
}

fn op_add(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    memory[0] = memory[addr] + value;

    return ip;
}

fn op_nand(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    return ip;
}

fn op_shl(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    memory[0] = memory[addr] << value;

    return ip;
}

fn op_shr(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    memory[0] = memory[addr] >> value;

    return ip;
}

fn op_jz(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    if flags & FLAG_ZERO != 0 {
        let ip = memory[addr];
    }

    return ip;
}

fn op_lt(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    memory[0] = memory[addr] < value;

    return ip;
}

fn op_gt(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    memory[0] = memory[addr] > value;

    return ip;
}

fn op_in(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    return ip;
}

fn op_out(ip: u16, flags: &u16, memory: &[u16], addr: u16, value: u16) -> u16 {
    match memory[addr] {
        0 => print!("{}", value as char), // display.
    }

    return ip;
}

fn op_undefined(ip: u16, flags: &u16, memory: &[u16], opcode: u16, addr: u16, value: u16) -> u16 {
    panic!("invalid opcode: {}", opcode);
}

fn dispatch(ip: u16, flags: &u16, memory: &[u16], opcode: u16, addr: u16, value: u16) -> u16 {
    match opcode {
        0b0000 => op_mov(ip, flags, &memory, one, two),
        0b0001 => op_add(ip, flags, &memory, one, two),
        0b0010 => op_nand(ip, flags, &memory, one, two),
        0b0011 => op_shl(ip, flags, &memory, one, two),
        0b0100 => op_shr(ip, flags, &memory, one, two),
        0b0101 => op_jz(ip, flags, &memory, one, two),
        0b0110 => op_lt(ip, flags, &memory, one, two),
        0b0111 => op_gt(ip, flags, &memory, one, two),
        // No 0b1000-0b1101.
        0b1110 => op_in(ip, flags, &memory, one, two),
        0b1111 => op_out(ip, flags, &memory, one, two),
        _      => op_undefined(ip, flags, &memory, opcode, one, two),
    }
}

fn run_program(program: Vec<u16>) {
    let mut ip: u16 = 0;
    let mut flags: u16 = 0;
    let mut state = [0u16; 1024]; // 1kb of memory.

    for i in 0..program.len() {
        state[i] = program[i];
    }

    loop {
        let opcode = state[ip as usize];
        let value1 = state[ip as usize + 1];
        let value2 = state[ip as usize + 2];

        if opcode & 0b0001_0000 != 0 { // if the Pointer modifier is set.
            let value2 = state[value2 as usize];
        }

        ip = dispatch(ip, &flags, &state, opcode, value1, value2);
    }
}

fn read_program_file(filename: String) -> Vec<u16> {
    // Read file as a series of u8s (despite it being u16's).dd
    let mut file = File::open(filename).unwrap();
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf).unwrap();

    // Recast as u16s.
    let mut reader = Cursor::new(buf);
    let mut program: Vec<u16> = vec![];
    loop {
        match reader.read_u16::<BigEndian>() {
            Ok(val) => {
                program.push(val);
            },
            Err(_) => {
                break;
            }
        }
    }

    program.clone()
}

fn print_help() {
    println!("Usage: escolar PROGRAM");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 || args[0] == "--help" || args[0] == "-h" {
        print_help();
        return;
    }

    run_program(read_program_file(args[1].clone()));
}
