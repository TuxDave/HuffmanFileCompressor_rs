use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub mod utils;

#[derive(PartialEq, Eq)]
struct HuffmanTreeNode {
    frequency: u32,
    symbol: Option<u8>,
    left: Option<Box<HuffmanTreeNode>>,
    right: Option<Box<HuffmanTreeNode>>,
}

impl HuffmanTreeNode {
    fn new_leaf(frequency: u32, symbol: u8) -> HuffmanTreeNode {
        HuffmanTreeNode {
            frequency,
            symbol: Some(symbol),
            left: None,
            right: None,
        }
    }

    fn new_root(
        frequency: u32,
        left: HuffmanTreeNode,
        right: HuffmanTreeNode,
    ) -> HuffmanTreeNode {
        HuffmanTreeNode {
            frequency,
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn get_compression(&self) -> Result<HashMap<u8, Vec<bool>>, String> {
        let mut map: HashMap<u8, Vec<bool>> = HashMap::new();

        fn explore(map: &mut HashMap<u8, Vec<bool>>, tree: &HuffmanTreeNode, path: Vec<bool>) -> Result<(), String> {
            if tree.is_leaf() {
                if let Some(symbol) = tree.symbol {
                    map.insert(symbol, path);
                    Ok(())
                } else {
                    Err(String::from("Bad Huffman Tree"))
                }
            } else {
                if let Some(left) = &tree.left {
                    if let Err(err) = explore(map, left, [path.clone(), vec!(false)].concat()) {
                        println!("{}", err);
                        return Err(err);
                    }
                }
                if let Some(right) = &tree.right {
                    if let Err(err) = explore(map, right, [path.clone(), vec!(true)].concat()) {
                        println!("{}", err);
                        return Err(err);
                    }
                }
                return Ok(());
            }
        }
        match explore(&mut map, self, Vec::new()) {
            Ok(_) => { Ok(map) }
            Err(s) => { Err(s) }
        }
    }
}

impl Ord for HuffmanTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.frequency.cmp(&other.frequency);
    }
}

impl PartialOrd for HuffmanTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.frequency.partial_cmp(&other.frequency)
    }
}

fn huffmanize(m: HashMap<u8, u32>) -> Option<HuffmanTreeNode> {
    let mut pq: BinaryHeap<HuffmanTreeNode> = BinaryHeap::new();

    m.iter().for_each(|(k, v)| {
        pq.push(HuffmanTreeNode::new_leaf(*v, *k))
    });

    while pq.len() > 1 {
        let n1 = pq.pop()?;
        let n2 = pq.pop()?;
        pq.push(HuffmanTreeNode::new_root(
            n1.frequency + n2.frequency,
            n1,
            n2
        ));
    }
    return pq.pop()
}