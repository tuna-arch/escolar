extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Write};
use std::io::Cursor;

const UNINITIALIZED_MEMORY: u16 = 0b1111_1111;

#[repr(u16)]
enum Opcode {
    MOV     = 0b0000,
    ADD     = 0b0001,
    NAND    = 0b0010,
    SHL     = 0b0011,
    SHR     = 0b0100,
    JZ      = 0b0101,
    LT      = 0b0110,
    GT      = 0b0111,
    _INVALID8 = 0b1000,
    _INVALID9 = 0b1001,
    _INVALID10 = 0b1010,
    _INVALID11 = 0b1100,
    HLT     = 0b1101,
    IN      = 0b1110,
    OUT     = 0b1111,
}

impl Into<u16> for Opcode {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for Opcode {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::MOV as u16 => Ok(Self::MOV),
            x if x == Self::ADD as u16 => Ok(Self::ADD),
            x if x == Self::NAND as u16 => Ok(Self::NAND),
            x if x == Self::SHL as u16 => Ok(Self::SHL),
            x if x == Self::SHR as u16 => Ok(Self::SHR),
            x if x == Self::JZ as u16 => Ok(Self::JZ),
            x if x == Self::LT as u16 => Ok(Self::LT),
            x if x == Self::GT as u16 => Ok(Self::GT),
            x if x == Self::HLT as u16 => Ok(Self::HLT),
            x if x == Self::IN as u16 => Ok(Self::IN),
            x if x == Self::OUT as u16 => Ok(Self::OUT),
            _ => Err(()),
        }
    }
}

//type Op = dyn Fn(u16, &u16, &[u16], u16, u16);

const FLAG_CARRY: u16 = 0b0001;
const FLAG_ZERO:  u16 = 0b0010;

fn op_mov(ip: u16, _flags: &u16, memory: &mut [u16], addr: u16, value: u16) -> u16 {
    memory[addr as usize] = value;

    return ip;
}

fn op_add(ip: u16, _flags: &u16, memory: &mut [u16], addr: u16, value: u16) -> u16 {
    memory[0] = memory[addr as usize] + value;

    return ip;
}

fn op_nand(ip: u16, _flags: &u16, memory: &mut [u16], addr: u16, value: u16) -> u16 {
    memory[0] = !((memory[addr as usize] != 1) && (value != 0)) as u16;

    return ip;
}

fn op_shl(ip: u16, _flags: &u16, memory: &mut [u16], addr: u16, value: u16) -> u16 {
    memory[0] = memory[addr as usize] << value;

    return ip;
}

fn op_shr(ip: u16, _flags: &u16, memory: &mut [u16], addr: u16, value: u16) -> u16 {
    memory[0] = memory[addr as usize] >> value;

    return ip;
}

fn op_jz(ip: u16, flags: &u16, memory: &mut [u16], addr: u16, _value: u16) -> u16 {
    if flags & FLAG_ZERO != 0 {
        return memory[addr as usize];
    }

    return ip;
}

fn op_lt(ip: u16, _flags: &u16, memory: &mut [u16], addr: u16, value: u16) -> u16 {
    if memory[addr as usize] < value {
        memory[0] = 1u16;
    } else {
        memory[0] = 0u16;
    }

    return ip;
}

fn op_gt(ip: u16, _flags: &u16, memory: &mut [u16], addr: u16, value: u16) -> u16 {
    if memory[addr as usize] > value {
        memory[0] = 1u16;
    } else {
        memory[0] = 0u16;
    }

    return ip;
}

fn op_in(ip: u16, _flags: &u16, _memory: &mut [u16], addr: u16, value: u16) -> u16 {
    println!("in ( addr={}, value={})", addr, value);
    //let mut buffer = String::new();
    //std::io::stdin().read_line(&mut buffer).unwrap();
    return ip;
}

fn op_out(ip: u16, _flags: &u16, memory: &mut [u16], addr: u16, value: u16) -> u16 {
    match memory[addr as usize] {
        0 => print!("{}", value as u8 as char), // display.
        _ => panic!("no such output device: {}", value),
    }

    std::io::stdout().flush().unwrap();

    return ip;
}

fn op_undefined(ip: u16, flags: &u16, _memory: &mut [u16], opcode: u16, addr: u16, value: u16) -> u16 {
    panic!("invalid opcode: {}\n  ip = {:?}\n  flags = {}\n  addr = {}\n  value = {}", opcode, ip, flags, addr, value);
}

fn dispatch(ip: u16, flags: &u16, memory: &mut [u16], opcode: u16, addr: u16, value: u16) -> u16 {
    if opcode == UNINITIALIZED_MEMORY {
        panic!("invalid opcode: {}\nescolar uses this for uninitialized memory — maybe you forgot a HLT command?", opcode);
    }

    let Ok(oc): Result<Opcode, _> = opcode.try_into() else {
        panic!("unknown opcode {}", opcode);
    };

    match oc {
        Opcode::MOV     => op_mov(ip, flags, memory, addr, value),
        Opcode::ADD     => op_add(ip, flags, memory, addr, value),
        Opcode::NAND    => op_nand(ip, flags, memory, addr, value),
        Opcode::SHL     => op_shl(ip, flags, memory, addr, value),
        Opcode::SHR     => op_shr(ip, flags, memory, addr, value),
        Opcode::JZ      => op_jz(ip, flags, memory, addr, value),
        Opcode::LT      => op_lt(ip, flags, memory, addr, value),
        Opcode::GT      => op_gt(ip, flags, memory, addr, value),
        // No 0b1000-0b1101.
        Opcode::IN      => op_in(ip, flags, memory, addr, value),
        Opcode::OUT     => op_out(ip, flags, memory, addr, value),
        Opcode::HLT     => { ip },
        _      => op_undefined(ip, flags, memory, opcode, addr, value),
    }
}

fn run_program(program: Vec<u16>) {
    let mut ip: u16 = 0;
    let mut flags: u16 = 0;
    let mut state = [UNINITIALIZED_MEMORY; 1024]; // 1kb of memory.

    for i in 0..program.len() {
        state[i] = program[i];
    }

    loop {
        let mut opcode = state[ip as usize];
        let addr   = state[ip as usize + 1];
        let mut value  = state[ip as usize + 2];

        if opcode == (Opcode::HLT as u16) {
            break
        }

        let is_invalid = opcode == UNINITIALIZED_MEMORY;
        let is_ptr = opcode & 0b0001_0000 != 0;
        if !is_invalid && is_ptr { // if the Pointer modifier is set.
            opcode -= 0b0001_0000;
            value = state[value as usize];
        }

        //let ptr_indicator = if is_ptr { "*" } else { " " };
        //println!("{} {} {} {}", ptr_indicator, );

        let old_ip = ip;
        ip = dispatch(ip, &flags, &mut state, opcode, addr, value);

        if state[0] == 0 {
            flags |= FLAG_ZERO;
        } else {
            // TODO: unset FLAG_ZERO
        }

        if ip == old_ip {
            ip += 3;
        }
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
