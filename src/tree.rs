
trait Hierarchical<T> {
	fn new(data: T) -> Self;
	fn get_data(&self) -> &T;
	fn add_child(&mut self, data: T) -> &Self;
}


#[derive(Debug)]
struct Node<T> {
	data: T,
	children: Vec<Node<T>>,
}


#[derive(Debug, PartialEq)]
struct Item {
	index: usize,
}


impl<T> Hierarchical<T> for Node<T> {
	fn new(data: T) -> Self {
		Node {
			children: vec!(),
			data: data,
		}
	}

	fn add_child(&mut self, data: T) -> &Self {
		let child = Self::new(data);
		self.children.push(child);
		&self.children[self.children.len()-1]
	}

	fn get_data(&self) -> &T {
		&self.data
	}
}



#[cfg(test)]
mod tests {
	use super::Hierarchical;
	use super::Node;
	use super::Item;

	#[test]
	fn new_node_test() {
		let item = Item { index: 0 };
		let hierarchy = Node::new(item);
		assert_eq!(hierarchy.get_data(), &Item{ index: 0 });
	}
	#[test]
	fn add_child_test() {
		let item = Item { index: 0 };
		assert_eq!(Node::new(item).add_child(Item { index: 1 } ).get_data(), &Item{ index: 1 });
	}
}
