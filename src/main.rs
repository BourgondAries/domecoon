#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(items_after_statements))]

extern crate acon;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

type Rcc<T> = Rc<RefCell<T>>;

#[derive(Debug)]
struct Genealogy {
	identifiers: BTreeMap<String, Individual>,
}

impl Genealogy {
	fn new() -> Genealogy {
		Genealogy {
			identifiers: BTreeMap::new(),
		}
	}

	/// Create a new individual with a unique identifier
	fn create_individual(&mut self, identifier: &str) -> Result<Individual, GeneError> {
		if self.identifiers.contains_key(identifier) {
			return Err(GeneError::IdentifierInUse);
		}
		let individual = Individual::new(identifier);
		self.identifiers.insert(identifier.to_string(), individual.clone());
		Ok(individual)
	}

	fn contains_individual(&self, identifier: &str) -> bool {
		self.identifiers.contains_key(identifier)
	}

	/// Retrieve the individual using the unique identifier
	fn get_individual(&self, identifier: &str) -> Option<&Individual> {
		self.identifiers.get(identifier)
	}

	/// Remove the individual entirely using the unique identifier
	/// Note that removal is not possible if the individual has parents or children
	fn remove_individual(&mut self, identifier: &str) -> Option<Individual> {

		self.identifiers.remove(identifier)
	}

	fn set_child(&self, parent: &str, child: &str) -> Result<(), GeneError> {
		match self.get_individual(parent) {
			Some(parent) => {
				let mut parent = parent.clone();
				match self.get_individual(child) {
					Some(child) => {
						let mut child = child.clone();
						parent.add_child(&mut child)
					}
					None => Err(GeneError::NoSuchChild),
				}
			}
			None => Err(GeneError::NoSuchParent),
		}
	}
}

#[derive(Debug)]
enum GeneError {
	IdentifierInUse,
	IsAlreadyAChild,
	NoSuchChild,
	NoSuchParent,
}

#[derive(Clone, Debug)]
struct IndividualData {
	children: Vec<Individual>,
	identifier: String,
	parents: Vec<Individual>,
}

#[derive(Clone, Debug)]
struct Individual(Rcc<IndividualData>);

impl Individual {
	fn new(identifier: &str) -> Individual {
		Individual (
			Rc::new(RefCell::new(
				IndividualData {
					children: vec![],
					identifier: identifier.to_string(),
					parents: vec![],
				}
			))
		)
	}

	/// Check if 'parent' is an ancestor of this individual
	fn is_ancestor(&self, child: &Individual) -> bool {
		let inner = self.0.borrow_mut();
		let child = child.0.borrow();
		let name = child.identifier.as_ref();
		for individual in inner.children.iter() {
			let temp = individual.0.borrow();
			if temp.identifier == name {
				return true;
			}
		}
		false
	}

	fn add_child(&mut self, child: &mut Individual) -> Result<(), GeneError> {
		if self.is_ancestor(child) {
			return Err(GeneError::IsAlreadyAChild);
		}
		let mut inner = self.0.borrow_mut();
		inner.children.push(child.clone());
		let mut child = child.0.borrow_mut();
		child.parents.push(self.clone());
		Ok(())
	}
}

#[test]
fn main() {
	let mut tree = Genealogy::new();
	tree.create_individual("Parent").unwrap();
	tree.create_individual("John").unwrap();
	tree.create_individual("Jane").unwrap();

	tree.set_child("Parent", "John").unwrap();
	tree.set_child("Parent", "Jane").unwrap();
}

#[test]
fn test() {
	assert!(true);
}
