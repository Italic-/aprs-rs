//! Frame Check Sequence
//!
//! The FCS is a sequence of 16 bits used for checking the integrity of a
//! received frame.
//!
//! Derived from [casebeer/afsk](https://github.com/casebeer/afsk)
//!
//! Copyright (c) 2013 Christopher H. Casebeer. All rights reserved.


use byteorder::{ByteOrder, LittleEndian};
use bit_vec::BitVec;


pub struct FCS {
    fcs: u16,
}

impl FCS {
    pub fn new() -> FCS {
        FCS {
            fcs: 0xFFFF
        }
    }
    pub fn update_bit(&mut self, bit: bool) {
        let check: bool = self.fcs & 0x1 == 1;
        self.fcs = self.fcs >> 1;
        if check != bit {
            self.fcs = self.fcs ^ 0x8408_u16;
        }
    }
    pub fn update_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            for i in (0..8).rev() {
                // pass each bit of u8 from left->right (true/false)
                self.update_bit(((byte >> i) & 0x01_u8) == 1_u8);
            }
        }
    }
    pub fn digest(&self) -> Vec<u8> {
        // Two bytes (u16), little endian
        let mut ret: Vec<u8> = Vec::new();
        LittleEndian::write_u16(&mut ret, !self.fcs % ::std::u16::MAX);
        ret
    }
}


pub fn fcs(bits: Vec<bool>) -> Vec<bool> {
    let mut fcs_sum: Vec<bool> = Vec::new();
    let mut fcs = FCS::new();
    for bit in &bits {
        fcs.update_bit(*bit);
    }
    fcs_sum.extend(&bits);

    // little-bit-endian in addition to little-byte-endian
    let mut bit_vec: Vec<u8> = Vec::new();
    for byte in fcs.digest().iter() {
        bit_vec.push(reverse_bits(*byte));
    }
    fcs_sum.extend(BitVec::from_bytes(&bit_vec));

    fcs_sum
}


pub fn fcs_validate(bits: Vec<bool>) -> Result<bool, String> {
    let mut buffer: Vec<bool> = Vec::new();
    let mut fcs = FCS::new();

    for bit in bits {
        buffer.push(bit);
        if buffer.len() > 16 {
            buffer = {
                let (first, seq) = buffer.split_at(1);
                fcs.update_bit(first[0]);
                seq.to_vec()
            };
        }
    }
    let mut _buffer: BitVec = BitVec::new();
    _buffer.extend(buffer.into_iter());

    if fcs.digest() != _buffer.to_bytes() {
        return Err(String::from("Invalid FCS"));
    }
    return Ok(true);
}


fn reverse_bits(bits: u8) -> u8 {
    // From https://graphics.stanford.edu/~seander/bithacks.html#BitReverseObvious
    let mut r: u8 = bits; // reversed bits of v; first get LSB of bits
    let mut s: u8 = 7;    // shift amount at end
    for i in 1..8 {
        let v: u8 = bits >> i;
        if v != 0 {
            r = r << 1;
            r = r | (v & 1);
            s = s - 1;
        }
    }
    r << s                // shift when bits' highest bits are 0
}
