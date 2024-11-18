use std::collections::{HashMap};
use std::fs::File;
use std::io::Read;

pub fn count_occurrence(f: &mut File) -> HashMap<u8, u32> {
    let mut occurrence: HashMap<u8, u32> = HashMap::new();

    let mut buff: [u8; 1] = [0];
    while f.read(&mut buff).unwrap_or(0) != 0 {
        let byte = buff[0];
        occurrence.insert(byte, if let Some(prev) = 
            occurrence.get(&byte) { prev + 1 } else { 1 });
    }
    occurrence
}

pub fn from_byte_to_bool(mut b: u8) -> [bool; 8] {
    let mut bools: [bool; 8] = [false; 8];
    for i in (0 ..= 7).rev() {
        if b % 2 == 1 {
            bools[i] = true;
        }
        b = b >> 1;
    }
    bools
}