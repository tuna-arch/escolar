type Op = Fn(u16, u16);

fn op_mov(one: u16, two: u16) {
}

fn op_add(one: u16, two: u16) {
}

fn op_nand(one: u16, two: u16) {
}

fn op_shl(one: u16, two: u16) {
}

fn op_shr(one: u16, two: u16) {
}

fn op_jz(one: u16, two: u16) {
}

fn op_lt(one: u16, two: u16) {
}

fn op_gt(one: u16, two: u16) {
}

fn op_in(one: u16, two: u16) {
}

fn op_out(one: u16, two: u16) {
}

fn op_undefined(one: u16, two: u16) {
}

fn dispatch(opcode: u16, one: u16, two: u16) {
    match opcode {
        0b0000 => op_mov(one, two),
        0b0001 => op_add(one, two),
        0b0010 => op_nand(one, two),
        0b0011 => op_shl(one, two),
        0b0100 => op_shr(one, two),
        0b0101 => op_jz(one, two),
        0b0110 => op_lt(one, two),
        0b0111 => op_gt(one, two),
        // No 0b1000-0b1101.
        0b1110 => op_in(one, two),
        0b1111 => op_out(one, two),
        _      => op_undefined(one, two),
    };
}

fn main() {
}
