use std::collections::BTreeMap;



let mut map = BTreeMap::new();
map.insert("alice", 100);
assert_eq!(map.get(&"alice"), Some(&100));
assert_eq!(map.get(&"bob"), None);

pub struct Pallet{
	balances: BTreeMap<String, u128>,
	
}

impl Pallet {
	pub fn new() -> Self {
		Self {
			balances: BTreeMap::new()
		}
	}

	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
		
	}
	
	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
		
	}
}


