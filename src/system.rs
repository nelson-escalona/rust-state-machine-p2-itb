use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountID, Nonce>,
}

impl<AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce>
where
	AccountID: Ord + Clone,
	BlockNumber: Zero + One + AddAssign + Copy,
	Nonce: Zero + One + Copy,
{
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &AccountID) {
		let nonce: Nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
		let new_nonce = nonce + Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<String, u32, u32>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
