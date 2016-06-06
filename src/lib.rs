#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(items_after_statements))]
#![allow(dead_code)]

extern crate acon;
extern crate array_tool;

use array_tool::vec::Intersect;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug)]
enum Sex {
	Female,
	Hermaphrodite,
	Male,
}

#[derive(Debug)]
struct Individual<T> {
	children: Vec<usize>,
	individual: T,
	parents: Vec<usize>,
	alive: bool,
	fertile: bool,
	sex: Option<Sex>,
	score: Option<usize>,
	coefficient_of_inbreeding: Option<f64>,
}

impl<T> Individual<T> {
	fn new(individual: T) -> Individual<T> {
		Individual {
			children: vec![],
			individual: individual,
			parents: vec![],
			alive: true,
			fertile: true,
			sex: None,
			score: None,
			coefficient_of_inbreeding: None,
		}
	}
}

#[derive(Debug)]
struct Genealogy<T> {
	genealogy: Vec<Individual<T>>,
}

fn above_one_is_one(value: usize) -> usize {
	if value > 1 {
		1
	} else {
		0
	}
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
		tree.add("Archduke Charles II of Austria".to_string(), Some(12), Some(13));
		tree.add("Anna".to_string(), Some(12), Some(13));

		// 20
		tree.add("Anna of Austria".to_string(), Some(16), Some(17));
		tree.add("Mary".to_string(), Some(19), None);

		// 22
		tree.add("Charles (Don Carlos)".to_string(), Some(14), Some(15));
		tree.add("Philip III".to_string(), Some(15), Some(20));
		tree.add("Margaret of Austria".to_string(), Some(18), Some(21));
		tree.add("Ferdinand II".to_string(), Some(18), Some(21));

		// 26
		tree.add("Philip IV".to_string(), Some(23), Some(24));
		tree.add("Maria Anna of Austria".to_string(), Some(23), Some(24));
		tree.add("Ferdinand III".to_string(), Some(25), None);

		// 29
		tree.add("Mariana of Austria".to_string(), Some(27), Some(28));

		// 30
		tree.add("Charles II".to_string(), Some(26), Some(29));

		tree
	}

	fn first_cousins() -> Genealogy<String> {
		let mut tree = Genealogy::<String>::new();

		// Root ancestors
		tree.add("A".to_string(), None, None);
		tree.add("B".to_string(), None, None);

		// 2
		tree.add("D".to_string(), Some(0), Some(1));
		tree.add("E".to_string(), Some(0), Some(1));

		// 4
		tree.add("G".to_string(), Some(2), None);
		tree.add("H".to_string(), Some(3), None);

		tree
	}

	fn second_cousins() -> Genealogy<String> {
		let mut tree = Genealogy::<String>::new();

		// Root ancestors
		tree.add("A".to_string(), None, None);
		tree.add("B".to_string(), None, None);

		// 2
		tree.add("D".to_string(), Some(0), Some(1));
		tree.add("E".to_string(), Some(0), Some(1));

		// 4
		tree.add("H".to_string(), Some(2), None);
		tree.add("I".to_string(), Some(3), None);

		// 6
		tree.add("K".to_string(), Some(4), None);
		tree.add("L".to_string(), Some(5), None);

		// 8
		tree.add("M".to_string(), Some(6), Some(7));

		tree
	}

	fn direct_relationship() -> Genealogy<String> {
		let mut tree = Genealogy::<String>::new();

		// Root ancestors
		tree.add("A".to_string(), None, None);
		tree.add("C".to_string(), Some(0), None);
		tree.add("E".to_string(), Some(1), None);
		tree.add("G".to_string(), Some(2), None);
		tree.add("I".to_string(), Some(3), None);

		tree
	}
}

impl<T> Genealogy<T> where T: std::fmt::Debug {
	fn new() -> Genealogy<T> {
		Genealogy {
			genealogy: vec![],
		}
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

	fn add(&mut self, identifier: T, father: Option<usize>, mother: Option<usize>) -> usize {
		let id = self.genealogy.len();
		self.genealogy.push(Individual::new(identifier));
		self.add_parent(id, father);
		self.add_parent(id, mother);
		id
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
		let mut ancestors = vec![id];
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
		// let _: Genealogy<String> = Genealogy::new();
	}

	#[test]
	fn dijkstra() {
		// let tree: Genealogy<String> = Genealogy::sample_tree();
		// let path = tree.dijkstra(22, 29);
		// println!("{:?}", path);
	}

	#[test]
	fn path_via() {
		// let tree: Genealogy<String> = Genealogy::sample_tree();
		// let path = tree.path_via_ancestor(26, 24, 29);
		// println!("{:?}", path);
	}

	#[test]
	fn coefficient_of_relationship() {
		// let tree = Genealogy::sample_tree();
		// println!("Path Via Ancestor: {:?}", tree.path_via_ancestor(26, 18, 29));
		// println!("Relationship coefficient: {:?}", tree.find_relationship(26, 29));
	}

	#[test]
	fn second_cousins_m_a_relationship() {
		// let tree = Genealogy::second_cousins();
		// assert_eq!(Some(0.125), tree.find_relationship(0, 8));
	}

	#[test]
	fn second_cousins_m_b_relationship() {
		// let tree = Genealogy::second_cousins();
		// assert_eq!(Some(0.125), tree.find_relationship(1, 8));
	}

	#[test]
	fn first_cousins() {
		// let tree = Genealogy::first_cousins();
		// println!("First cousin: {:?}", tree.find_relationship(4, 5));
	}

	#[test]
	fn direct_relationship() {
		// let tree = Genealogy::direct_relationship();
		// assert_eq!(Some(0.0625), tree.find_relationship(0, 4));
	}
}
