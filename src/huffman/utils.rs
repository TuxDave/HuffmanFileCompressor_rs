use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

pub fn count_occurrence(f: &mut File) -> HashMap<u8, u32> {
    let mut occurrence: HashMap<u8, u32> = HashMap::new();

    let mut buff: [u8; 1] = [0];
    while (f.read(&mut buff).unwrap_or(0) != 0) {
        let byte = buff[0];
        occurrence.insert(byte, if let Some(prev) = occurrence.get(&byte) {prev + 1} else {1});
    }
    occurrence
    //TODO: to be tested
}