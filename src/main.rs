use huffman_file_compressor_rs::huffman::hello_huffman;
use huffman_file_compressor_rs::huffman::utils::{to_queue};

fn main() {
    let mut queue = to_queue(&vec![true]);
    println!("{:?}", queue.pop_front().unwrap());
    println!("{:?}", queue.pop_front().unwrap());
}