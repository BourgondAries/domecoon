#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(items_after_statements))]
#![allow(dead_code)]

extern crate acon;
extern crate array_tool;

use array_tool::vec::Intersect;

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

	fn siblings() -> Genealogy<String> {
		let mut tree = Genealogy::<String>::new();

		// Root ancestors
		tree.add("A".to_string(), None, None);
		tree.add("B".to_string(), None, None);

		// Siblings
		tree.add("C".to_string(), Some(0), Some(1));
		tree.add("D".to_string(), Some(0), Some(1));

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

	fn first_cousins_children() -> Genealogy<String> {
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

		// 6
		tree.add("I".to_string(), Some(4), Some(5));
		tree.add("J".to_string(), Some(4), Some(5));

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

	fn double_relationship() -> Genealogy<String> {
		let mut tree = Genealogy::<String>::new();

		// Root ancestors
		tree.add("A".to_string(), None, None);
		tree.add("B".to_string(), None, None);
		tree.add("C".to_string(), None, None);
		tree.add("D".to_string(), None, None);
		tree.add("E".to_string(), None, None);
		tree.add("F".to_string(), None, None);

		// 6
		tree.add("G".to_string(), Some(0), Some(1));
		tree.add("H".to_string(), Some(1), Some(2));
		tree.add("I".to_string(), Some(3), Some(4));
		tree.add("J".to_string(), Some(4), Some(5));

		// 10
		tree.add("K".to_string(), Some(6), Some(8));
		tree.add("L".to_string(), Some(7), Some(9));

		tree
	}

	fn diamond_relationship() -> Genealogy<String> {
		let mut tree = Genealogy::<String>::new();

		// Root ancestors
		tree.add("A".to_string(), None, None);

		tree.add("B".to_string(), Some(0), None);
		tree.add("C".to_string(), Some(0), None);

		tree.add("D".to_string(), Some(1), Some(2));

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

	fn add_parent(&mut self, id: usize, pid: Option<usize>) -> bool {
		if let Some(pid) = pid {
			if self.is_descendant_of(pid, id) {
				false
			} else {
				self.genealogy
					.get_mut(pid)
					.map_or_else(
						|| println!("Could not add child, unknown parent id"),
						|individual| individual.children.push(id));
				self.genealogy
					.get_mut(id)
					.map_or_else(
						|| println!("Could not add parent, unknown child id"),
						|individual| individual.parents.push(pid));
				true
			}
		} else {
			true
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

	fn is_descendant_of(&self, ancestor: usize, descendant: usize) -> bool {
		self.get_paths_from_ancestor_to_descendant(ancestor, descendant)
			.is_empty() == false
	}

	fn compute_coefficient_of_relationship(&self, id1: usize, id2: usize) -> Option<f64> {
		let ancestors1 = self.get_ancestors(id1);
		let ancestors2 = self.get_ancestors(id2);
		let common_ancestors = ancestors1.intersect(ancestors2);
		let mut coefficient: f64 = 0.0;
		for ancestor in common_ancestors {
			let mut paths_1 = self.get_paths_from_ancestor_to_descendant(
				ancestor,
				id1
			);
			let paths_2 = self.get_paths_from_ancestor_to_descendant(
				ancestor,
				id2
			);
			for path_1 in paths_1.iter_mut() {
				path_1.pop();
				for path_2 in paths_2.iter() {
					if path_1.intersect(path_2.clone()).is_empty() {
						println!("Found compatible paths: {:?}, and {:?}", path_1, path_2);
						coefficient += 0.5f64.powi(((path_1.len() + path_2.len()) - 1) as i32);
					}
				}
			}
		}
		Some(coefficient)
	}

	/// Find all paths from an ancestors to a descendant.
	///
	/// An empty outer Vec means there are no paths
	/// An inner vector is represented by
	/// [descendant, middle, ..., ancestor]
	/// A single [[value]] means that the ancestor needs
	/// no intermediary steps. It is already the requested
	/// descendant.
	fn get_paths_from_ancestor_to_descendant(&self, ancestor: usize, descendant: usize) -> Vec<Vec<usize>> {
		if ancestor == descendant {
			return vec![vec![ancestor]];
		}
		let mut paths = self
			.get_paths_from_ancestor_to_descendant_internal(
				ancestor, descendant
			);
		for path in paths.iter_mut() {
			path.push(ancestor);
		}
		paths
	}

	fn get_paths_from_ancestor_to_descendant_internal(&self, ancestor: usize, descendant: usize) -> Vec<Vec<usize>> {
		// We use Depth-First Search to find each path
		let mut returner = vec![];
		if let Some(children) = self.genealogy
		.get(ancestor)
		.map(|id| &id.children) {
			for child in children {
				if *child == descendant {
					returner.push(vec![*child]);
				} else {
					let mut prep = self.get_paths_from_ancestor_to_descendant_internal(
						*child, descendant
					);
					for i in prep.iter_mut() {
						i.push(*child);
					}
					for x in prep {
						returner.push(x);
					}
				}
			}
		}
		returner
	}

	/// Get all ancestors of this animal within a specified range
	/// The range is there to avoid tracking a large tree
	/// The coefficient of inbreeding is extremely small sufficiently far
	/// away and can be ignored
	fn get_ancestors(&self, id: usize) -> Vec<usize> {
		let mut ancestors = vec![id];
		let mut temp_ancestors = vec![id];
		let mut temp_ancestors_build = vec![];
		while temp_ancestors.is_empty() == false {
			for ancestor in &temp_ancestors {
				self.genealogy
				.get(*ancestor)
				.map(|id| &id.parents)
				.map(|parents| {
					ancestors.extend(parents.iter());
					temp_ancestors_build.extend(parents.iter());
				});
			}
			temp_ancestors = temp_ancestors_build;
			temp_ancestors_build = vec![];
		}
		ancestors.sort();
		ancestors
	}
}

#[cfg(test)]
mod tests {
	use super::Genealogy;

	#[test]
	fn second_cousins_k_l_relationship() {
		let tree = Genealogy::second_cousins();
		let relationship = tree.compute_coefficient_of_relationship(6, 7);
		assert_eq!(Some(0.03125), relationship);
	}

	#[test]
	fn second_cousins_m_d_relationship() {
		let tree = Genealogy::second_cousins();
		let relationship = tree.compute_coefficient_of_relationship(2, 8);
		assert_eq!(Some(0.1875), relationship);
	}

	#[test]
	fn second_cousins_m_a_relationship() {
		let tree = Genealogy::second_cousins();
		let relationship = tree.compute_coefficient_of_relationship(0, 8);
		assert_eq!(Some(0.125), relationship);
	}

	#[test]
	fn second_cousins_m_b_relationship() {
		let tree = Genealogy::second_cousins();
		let relationship = tree.compute_coefficient_of_relationship(1, 8);
		assert_eq!(Some(0.125), relationship);
	}

	#[test]
	fn first_cousins() {
		let tree = Genealogy::first_cousins();
		let relationship = tree.compute_coefficient_of_relationship(4, 5);
		assert_eq!(Some(0.125), relationship);
	}

	#[test]
	fn first_cousins_children() {
		let tree = Genealogy::first_cousins_children();
		let relationship = tree.compute_coefficient_of_relationship(6, 7);
		assert_eq!(Some(0.5625), relationship);
	}

	#[test]
	fn direct_relationship() {
		let tree = Genealogy::direct_relationship();
		let relationship = tree.compute_coefficient_of_relationship(0, 4);
		assert_eq!(Some(0.0625), relationship);
	}

	#[test]
	fn sibling_relationship() {
		let tree = Genealogy::direct_relationship();
		let relationship = tree.compute_coefficient_of_relationship(2, 3);
		assert_eq!(Some(0.5), relationship);
	}

	#[test]
	fn diamond_relationship() {
		let tree = Genealogy::diamond_relationship();
		let relationship = tree.compute_coefficient_of_relationship(0, 3);
		assert_eq!(Some(0.5), relationship);
	}

	#[test]
	fn double_relationship() {
		let tree = Genealogy::double_relationship();
		let relationship = tree.compute_coefficient_of_relationship(10, 11);
		assert_eq!(Some(0.125), relationship);
	}

	#[test]
	fn double_relationship_same() {
		let tree = Genealogy::double_relationship();
		let relationship = tree.compute_coefficient_of_relationship(11, 11);
		assert_eq!(Some(1.0), relationship);
	}

	#[test]
	fn is_ancestor() {
		let tree = Genealogy::direct_relationship();
		let descendant = tree.is_descendant_of(2, 3);
		assert_eq!(true, descendant);
		let descendant = tree.is_descendant_of(3, 2);
		assert_eq!(false, descendant);
	}

	#[test]
	fn is_ancestor_first_cousins() {
		let tree = Genealogy::first_cousins();
		let descendant = tree.is_descendant_of(0, 2);
		assert_eq!(true, descendant);
		let descendant = tree.is_descendant_of(4, 0);
		assert_eq!(false, descendant);
	}
}
