#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(items_after_statements))]

extern crate acon;
extern crate array_tool;

#[derive(Debug)]
struct Individual<T> {
	children: Vec<usize>,
	individual: T,
	parents: Vec<usize>,
}

impl<T> Individual<T> {
	fn new(individual: T) -> Individual<T> {
		Individual {
			children: vec![],
			individual: individual,
			parents: vec![],
		}
	}
}

#[derive(Debug)]
struct Genealogy<T> {
	genealogy: Vec<Individual<T>>,
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

		tree.print_nice();
		println!("{:?}", tree.get_ancestors(4, 8));
		println!("{:?}", tree.get_ancestors(5, 8));

		use array_tool::vec::Intersect;
		let ancestors = tree.get_ancestors(4, 8);
		let intersect = ancestors.intersect(tree.get_ancestors(5, 8));
		println!("{:?}", intersect);

	}
}
