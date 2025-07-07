use ethrex_common::types::Block;
use mojave_chain_utils::unique_heap::UniqueHeapItem;

#[derive(Debug, Clone)]
pub struct OrderedBlock(pub Block);

impl PartialEq for OrderedBlock {
    fn eq(&self, other: &Self) -> bool {
        self.0.header.number == other.0.header.number
    }
}

impl Eq for OrderedBlock {}

impl PartialOrd for OrderedBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.header.number.cmp(&other.0.header.number)
    }
}

impl UniqueHeapItem<u64> for OrderedBlock {
    fn key(&self) -> u64 {
        self.0.header.number
    }
}
