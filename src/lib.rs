#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(items_after_statements))]
#![allow(dead_code)]

extern crate acon;
extern crate array_tool;

#[derive(Debug)]
struct Individual<T> {
	children: Vec<usize>,
	individual: T,
	parents: Vec<usize>,
	fertile: bool,
	male: Option<bool>,
	score: Option<usize>,
	coefficient_of_inbreeding: Option<f64>,
}

impl<T> Individual<T> {
	fn new(individual: T) -> Individual<T> {
		Individual {
			children: vec![],
			individual: individual,
			parents: vec![],
			fertile: true,
			male: None,
			score: None,
			coefficient_of_inbreeding: None,
		}
	}
}

#[derive(Debug)]
struct Genealogy<T> {
	genealogy: Vec<Individual<T>>,
}

impl Genealogy<String> {
	fn sample_tree() -> Genealogy<String> {
		let mut tree = Genealogy::<String>::new();
		// Root ancestors
		tree.add("Ferdinand of Aragon".to_string(), None, None);
		tree.add("Elizabeth of Castile".to_string(), None, None);
		tree.add("Maximilian I".to_string(), None, None);
		tree.add("Mary of Burgundy".to_string(), None, None);
		tree.add("Manuel I".to_string(), None, None);

		// 5
		tree.add("Mary of Aragon".to_string(), Some(0), Some(1));
		tree.add("Joanna I".to_string(), Some(0), Some(1));
		tree.add("Philip I".to_string(), Some(2), Some(3));

		// 8
		tree.add("John III".to_string(), Some(4), Some(5));
		tree.add("Isabella of Portugal".to_string(), Some(4), Some(5));
		tree.add("Catherine".to_string(), Some(6), Some(7));
		tree.add("Charles I".to_string(), Some(6), Some(7));
		tree.add("Ferdinand I".to_string(), Some(6), Some(7));
		tree.add("Anna of Hungary".to_string(), None, None);

		// 14
		tree.add("Mary of Portugal".to_string(), Some(8), Some(10));
		tree.add("Philip II".to_string(), Some(9), Some(11));
		tree.add("Maria".to_string(), Some(9), Some(11));
		tree.add("Maximilian II".to_string(), Some(12), Some(13));
		tree.add("Archduke Charles II".to_string(), Some(12), Some(13));
		tree.add("Anna".to_string(), Some(12), Some(13));

		// 20
		tree.add("Anna of Austria".to_string(), Some(16), Some(17));
		tree.add("Mary".to_string(), Some(19), None);

		// 22
		tree.add("Charles".to_string(), Some(14), Some(15));
		tree.add("Philip III".to_string(), Some(15), Some(20));
		tree.add("Margaret of Austria".to_string(), Some(18), Some(21));
		tree.add("Ferdinand II".to_string(), Some(18), Some(21));

		// 26
		tree.add("Philip IV".to_string(), Some(23), Some(24));
		tree.add("Maria Anna of Austria".to_string(), Some(23), Some(24));
		tree.add("Ferdinand III".to_string(), Some(23), None);

		// 29
		tree.add("Mariana of Austria".to_string(), Some(27), Some(28));

		// 30
		tree.add("Charles II".to_string(), Some(26), Some(29));

		tree
	}


}

impl<T> Genealogy<T> where T: std::fmt::Debug {
	fn new() -> Genealogy<T> {
		Genealogy {
			genealogy: vec![],
		}
	}

	fn dijkstra(&self, from: usize, to: usize) -> Option<Vec<usize>> {
		use std::collections::{BTreeMap, BTreeSet};

		let mut seen: BTreeSet<usize> = BTreeSet::new();
		let mut backtrace: BTreeMap<usize, usize> = BTreeMap::new();
		let mut active: Vec<usize> = vec![from];
		let mut stage: Vec<usize> = vec![];

		'looper: while active.is_empty() == false {
			for key in active.iter() {
				let individual = match self.genealogy.get(*key) {
					Some(individual) => individual,
					None => return None,
				};
				stage.extend(individual.children.iter()
																				.chain(individual.parents.iter())
																				.filter(|x| !seen.contains(x))
																				.inspect(|x| { backtrace.insert(**x, *key); }));
				seen.extend(stage.iter());
			}
			active = stage;
			stage = vec![];
			if seen.contains(&to) {
				break 'looper;
			}
		}

		let mut path = vec![to];
		loop {
			let last_id = match path.last() {
				Some(last) => *last,
				None => return None,
			};
			let next = match backtrace.get(&last_id) {
				Some(next) => next,
				None => return None,
			};
			path.push(*next);
			if *next == from {
				break;
			}
		}
		println!("{:?}", backtrace);
		Some(path)
	}

	fn exists(&self, id: usize) -> bool {
		self.genealogy.len() < id
	}

	fn add_parent(&mut self, id: usize, pid: Option<usize>) {
		if let Some(pid) = pid {
			match self.genealogy.get_mut(pid) {
				Some(ref mut individual) => individual.children.push(id),
				None => println!("Could not add child, unknown parent id"),
			}
			match self.genealogy.get_mut(id) {
				Some(ref mut individual) => individual.parents.push(pid),
				None => println!("Could not add parent, unknown child id"),
			}
		}
	}

	fn add(&mut self, identifier: T, father: Option<usize>, mother: Option<usize>) {
		let id = self.genealogy.len();
		self.genealogy.push(Individual::new(identifier));
		self.add_parent(id, father);
		self.add_parent(id, mother);
	}

	fn tail(&self, amount: usize) -> Option<&[Individual<T>]> {
		if amount >= self.genealogy.len() {
			None
		} else {
			Some(&self.genealogy[self.genealogy.len() - amount..])
		}
	}

	fn print_nice(&self) {
		let mut counter = 0;
		while let Some(i) = self.genealogy.get(counter) {
			println!("id: {}, children: {:?}, parents: {:?}, individual: {:?}",
				counter, i.children, i.parents, i.individual);
			counter += 1;
		}
	}

	/// Get all ancestors to this animal within a specified range
	/// The range is there to avoid tracking a large tree
	/// The coefficient of inbreeding is extremely small sufficiently far
	/// away and can be ignored
	fn get_ancestors(&self, id: usize, max_depth: usize) -> Vec<usize> {
		let mut current = 0;
		let mut ancestors = vec![];
		let mut temp_ancestors = vec![id];
		let mut temp_ancestors_build = vec![];
		while current < max_depth {
			for ancestor in &temp_ancestors {
				match self.genealogy.get(*ancestor).map(|id| &id.parents) {
					Some(parents) => {
						for i in parents {
							ancestors.push(*i);
							temp_ancestors_build.push(*i);
						}
					}
					None => {}
				}
			}
			temp_ancestors = temp_ancestors_build;
			temp_ancestors_build = vec![];
			current += 1;
		}
		ancestors.sort();
		ancestors
	}
}

#[cfg(test)]
mod tests {
	use super::Genealogy;
	#[test]
	fn build_simple() {
		let mut tree = Genealogy::new();

		tree.add("Coony", None, None);
		tree.add("Washy", None, None);
		tree.add("Judy", Some(0), Some(1));
		tree.add("Jamey", Some(0), Some(1));
		tree.add("Billy", Some(2), Some(3));
		tree.add("Jilly", Some(4), Some(0));

		// tree.print_nice();
		// println!("{:?}", tree.get_ancestors(4, 8));
		// println!("{:?}", tree.get_ancestors(5, 8));

		use array_tool::vec::Intersect;
		let ancestors = tree.get_ancestors(4, 8);
		let intersect = ancestors.intersect(tree.get_ancestors(5, 8));
		// println!("{:?}", intersect);

	}

	#[test]
	fn dijkstra() {
		let tree: Genealogy<String> = Genealogy::sample_tree();

		tree.print_nice();
		let path = tree.dijkstra(30, 0);
		println!("{:?}", path);
	}
}
